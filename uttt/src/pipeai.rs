use crate::ai::AI;
use std::process::{Command, Stdio, Child};
use std::io::Write;
use std::io::Read;

pub struct PipeAI {
    process: Child,
}

impl AI for PipeAI {
    fn get_move(&mut self, last_move: i64) -> i64 {
        let to_send = last_move.to_string() + "\r\n";
        match self.process.stdin.as_mut().unwrap().write(to_send.as_bytes()) {
            Err(why) => panic!("couldn't write to AI: {}", why),
            Ok(_) => (),
        }
        self.process.stdin.as_mut().unwrap().flush().unwrap();
        //println!("sent {} successfully", to_send);
        let mut res_vec : Vec<u8> = vec![32; 100];
        match self.process.stdout.as_mut().unwrap().read(&mut res_vec[..]) {
            Err(_) => panic!("couldn't read from AI:"),
            Ok(_) => (),
        };
        let untrimmed_response = String::from_utf8(res_vec).unwrap();
        let response = untrimmed_response.trim();
        //println!("received {} successfully", response);
        return match response.parse::<i64>() {
            Err(_) => -1,
            Ok(t) => t,
        }
    }

    fn cleanup(&mut self) {
        self.process.kill();
    }
}

impl PipeAI {
    pub fn new(cmd: String) -> PipeAI {
        PipeAI {
            process: match Command::new(cmd.clone())
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn() {
                Err(why) => panic!("couldn't spawn {}: {:?}", cmd, why),
                Ok(process) => process,
            },
        }
    }
}