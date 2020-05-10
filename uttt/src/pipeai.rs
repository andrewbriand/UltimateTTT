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
        self.process.stdin.as_mut().unwrap().flush();
        println!("sent {} successfully", to_send);
        let mut res_vec : Vec<u8> = Vec::with_capacity(100);
        match self.process.stdout.as_mut().unwrap().read(&mut res_vec[..]) {
            Err(why) => panic!("couldn't read from AI:"),
            Ok(_) => (),
        }
        let response = String::from_utf8(res_vec).unwrap();
        println!("received {} successfully", response);
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
                Err(why) => panic!("couldn't spawn {}: {:?}", cmd, why),
                Ok(process) => process,
            },
        }
    }
}