use crate::ai::AI;
use crate::uti::parse_info;

use std::process::Stdio;
use std::time::{Duration, Instant};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::timeout;
use tokio::time::Elapsed;
use async_trait::async_trait;

pub struct PipeAI {
    process: Child,
    cstdout: BufReader<ChildStdout>, // child stdout
    cstdin: ChildStdin,              // child stdin
    rem_time: Duration,
}

impl PipeAI {
    pub fn new(cmd: String, args: Vec<String>, init_time: Duration) -> PipeAI {
        let mut command = Command::new(cmd.clone());
        let command = command.args(&args[..]);
        let command = command.kill_on_drop(true);
        command.stdout(Stdio::piped());
        command.stdin(Stdio::piped());

        let mut proc = command.spawn().expect("could not spawn child process");
        let stdout = proc
            .stdout
            .take()
            .expect("child process does not have stdout");
        let stdin = proc
            .stdin
            .take()
            .expect("child process does not have stdin");
        PipeAI {
            process: proc,
            cstdout: BufReader::new(stdout),
            cstdin: stdin,
            rem_time: init_time,
        }
    }

    /*
    // returns true if is running, and false if not or took to long to return
    fn is_running(&self, time_allowed: Duration) -> bool {
        timed!(time_allowed, self.process.status())
    }
    */

    // if was able to read a line in time_allowed, populate buf with the result and returned time taken.I
    // Otherwise return Err<Elapsed>
    // Also trims it
    async fn timed_rdline(
        &mut self,
        time_allowed: Duration,
        buf: &mut String,
    ) -> Result<Duration, Elapsed> {
        let start = Instant::now();
        // TODO do we want specific precision for this check, e.g. miliseconds?
        let result = timeout(time_allowed, self.cstdout.read_line(buf)).await;
        match result {
            Ok(n) => {
                // TODO probably count as loss
                let n = n.expect("IO error when reading from child stdout");
                if n == 0 {
                    eprintln!("warning: reached EOF on a child stdout");
                }
                *buf = String::from(buf.trim());
                return Ok(start.elapsed()); // returned in time
            }
            Err(e) => return Err(e), // did not return in time
        }
    }

    // blocking write line to
    async fn send_command(&mut self, to_write: String) {
        let to_write = format!("{}{}", to_write, "\n");
        self.cstdin.write(to_write.as_bytes()).await.unwrap_or_else(
            |why| panic!("couldn't write to AI: {}", why)
        );
        self.cstdin.flush().await.unwrap_or_else(
            |why| panic!("failed to flush: {:?}", why)
        );
    }
}

#[async_trait]
impl AI for PipeAI {
    async fn ready(&mut self, time_allowed: Duration) -> bool {
        self.send_command(String::from("uti")).await;
        let mut buf = String::new();
        let res = self.timed_rdline(time_allowed, &mut buf).await;
        return match res {
            Ok(_) => {
                if buf != "utiok" {
                    eprintln!("child did not respond with 'utiok', instead responded with '{}'. Disqualified.", buf);
                    return false;
                }
                return true;
            }
            Err(_) => false, // did not return in time
        };
    }

    // currently only supports getting the best move.
    async fn get_move(&mut self, last_move: i64) -> i64 {
        // update last move
        if (last_move != -1) {
            self.send_command(format!("pos moves {}", last_move)).await;
        }
        self.send_command("search free".to_string()).await;

        // get move made
        let mut buf = String::new();
        let res = self.timed_rdline(self.rem_time, &mut buf).await;
        // TODO have get_move return Result and have the parent scope handle
        // timeout or parsing errors
        // ALso this is not good syntax but I think three levels of match is worse
        if res.is_err() {
            eprintln!("timed out");
            return -1;
        }

        // subtract time taken
        self.rem_time -= res.unwrap();

        // parse best move
        let map = parse_info(&buf[..]);
        match map {
            Err(why) => {
                // TODO again, return Err and let outer scope handle
                eprintln!("could not parse info command '{}'; reason: '{}'. Counted as loss", buf, why);
                return -1;
            },
            Ok(map) => {
                if !map.contains_key("best_move") {
                    // TODO again, return Err and let outer scope handle
                    eprintln!("info does not contain best_move. Counted as loss");
                    return -1;
                }
                let best_move: u64 = match map["best_move"].parse() {
                    Ok(n) => {
                        self.send_command(format!("pos moves {}", n)).await;
                        n
                    },
                    Err(_) => {
                        // TODO again, return Err and let outer scope handle
                        eprintln!("could not parse best move '{}'. Counted as loss", map["best_move"]);
                        return -1;
                    }
                };
                return best_move as i64;
            }
        };
    }

    fn cleanup(&mut self) {}
}
