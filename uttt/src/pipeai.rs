use crate::ai::AI;
use crate::uti::parse_info;

use std::process::Stdio;
use std::time::{Duration, Instant};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, AsyncReadExt};
use tokio::process::{Child, ChildStdin, ChildStdout, ChildStderr, Command};
use tokio::time::timeout;
use tokio::time::Elapsed;
use async_trait::async_trait;

use std::thread::sleep;

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
        let to_write = format!("{}\n", to_write);
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
    async fn get_move(&mut self, last_move: i64, rem_time_x: Duration, rem_time_o: Duration) -> i64 {
        // update last move
        if last_move != -1 {
            self.send_command(format!("pos moves {}", last_move)).await;
        }
        self.send_command(format!("search free {} {}", rem_time_x.as_millis(), rem_time_o.as_millis())).await;

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

    fn get_rem_time(&self) -> Duration {
        self.rem_time
    }

    fn cleanup(&mut self) {}
}

pub struct CGPipeAI {
    process: Child,
    cstdout: BufReader<ChildStdout>, // child stdout
    cstdin: ChildStdin,              // child stdin
    //cstderr: BufReader<ChildStderr>,
    first_move: bool,
}

impl CGPipeAI {
    pub fn new(cmd: String, args: Vec<String>) -> CGPipeAI {
        let mut command = Command::new(cmd.clone());
        let command = command.args(&args[..]);
        let command = command.kill_on_drop(true);
        command.stdout(Stdio::piped());
        command.stdin(Stdio::piped());
        command.stderr(Stdio::piped());  // do nothing with the pipe to suppress stderr

        let mut proc = command.spawn().expect("could not spawn child process");
        let stdout = proc
            .stdout
            .take()
            .expect("child process does not have stdout");
        let stdin = proc
            .stdin
            .take()
            .expect("child process does not have stdin");
        // let stderr = proc
        //     .stderr
        //     .take()
        //     .expect("child process does not have stderr");
        CGPipeAI {
            process: proc,
            cstdout: BufReader::new(stdout),
            cstdin: stdin,
            //cstderr: BufReader::new(stderr),
            first_move: true,
        }
    }

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
        let to_write = format!("{}\n", to_write);
        self.cstdin.write(to_write.as_bytes()).await.unwrap_or_else(
            |why| panic!("couldn't write to AI: {}", why)
        );
        self.cstdin.flush().await.unwrap_or_else(
            |why| panic!("failed to flush: {:?}", why)
        );
    }
}

#[async_trait]
impl AI for CGPipeAI {
    async fn ready(&mut self, time_allowed: Duration) -> bool {
        sleep(time_allowed / 2);
        return true;
    }

    // currently only supports getting the best move.
    async fn get_move(&mut self, last_move: i64, rem_time_x: Duration, rem_time_o: Duration) -> i64 {
        // update last move
        let row;
        let col;
        if last_move == -1 {
            row = -1;
            col = -1;
        } else {
            let big = last_move / 9;
            let small = last_move % 9;
            row = (big / 3) * 3 + (small / 3);
            col = (big % 3) * 3 + (small % 3);
        }
        self.send_command(format!("{} {}\n0", row, col)).await;

        // get move made
        let mut buf = String::new();
        let rem_millis: u64;
        if self.first_move {
            rem_millis = 1000;
            self.first_move = false;
        } else {
            rem_millis = 100;
        }
        let res = self.timed_rdline(Duration::from_millis(rem_millis), &mut buf).await;
        // TODO have get_move return Result and have the parent scope handle
        // timeout or parsing errors
        // ALso this is not good syntax but I think three levels of match is worse
        if res.is_err() {
            eprintln!("timed out");
            return -1;
        }

        let tokens: Vec<&str> = buf.split_whitespace().collect();
        if tokens.len() != 2 && tokens.len() != 3 {
            eprintln!("Incorrect number of tokens returned: {}", tokens.len());
            return -2;
        }
        let row: u32 = tokens[0].parse().expect(&format!("Failed to parse row returned: '{}'", tokens[0]));
        let col: u32 = tokens[1].parse().expect(&format!("Failed to parse col returned: '{}'", tokens[1]));
        // TODO confirm legal
        let big_row = row / 3;
        let small_row = row % 3;
        let big_col = col / 3;
        let small_col = col % 3;

        return ((big_row * 3 + big_col) * 9 + (small_row * 3 + small_col)) as i64;
    }

    fn get_rem_time(&self) -> Duration {
        Duration::from_millis(0)
    }

    fn cleanup(&mut self) {}
}