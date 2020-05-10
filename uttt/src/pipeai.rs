use crate::ai::AI;
use std::process::{Command, Stdio, Child};
use std::io::Write;
use std::io::Read;

pub struct PipeAI {
    process: Child,
}

impl AI for PipeAI {
    fn get_move(&mut self, last_move: i64) -> i64 {
        let to_send = last_move.to_string();
        match self.process.stdin.as_mut().unwrap().write(to_send.as_bytes()) {
            Err(why) => panic!("couldn't write to AI: {}"),
            Ok(_) => (),
        }
        let mut response = String::new();
        match self.process.stdout.as_mut().unwrap().read_to_string(&mut response) {
            Err(why) => panic!("couldn't read from AI:"),
            Ok(_) => (),
        }
        return match response.parse::<i64>() {
            Err(E) => -1,
            Ok(T) => T,
        }
    }
}

impl PipeAI {
    pub fn new(cmd: String) -> PipeAI {
        PipeAI {
            process: match Command::new(cmd.clone())
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn() {
                Err(why) => panic!("couldn't spawn {}", cmd),
                Ok(process) => process,
            },
        }
    }
}