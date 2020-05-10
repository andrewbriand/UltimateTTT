use structopt::StructOpt;
mod board;
mod ai;
use ai::AI;
pub use board::Board;
mod humanplayer;
mod pipeai;
pub use pipeai::PipeAI;
pub use humanplayer::HumanPlayer;
use board::Player;
//use text_io::read;
//use std::time::Instant;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    o_ai_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    x_ai_path: std::path::PathBuf,
}

fn main() {
    let level = 2;
    let mut board = Board::new(level);
    //let mut x_ai = HumanPlayer::new(level);
    let mut x_ai = PipeAI::new("C:/ultimate-tictactoe/target/debug/main.exe".to_string());
    let mut o_ai = PipeAI::new("C:/ultimate-tictactoe/target/debug/main.exe".to_string());
    let mut last_move = x_ai.get_move(-1);
    loop {
        if last_move == -1 {
            println!("X forfeited");
            board.winner = Player::O;
            break;
        }
        if !board.make_move(last_move as usize) {
            println!("X made an illegal move");
            board.winner = Player::O;
            break;
        }
        if board.winner != Player::NEITHER {
            break;
        }
        last_move = o_ai.get_move(last_move);
        if last_move == -1 {
            println!("O forfeited");
            board.winner = Player::X;
            break;
        }
        if !board.make_move(last_move as usize) {
            println!("O made an illegal move");
            board.winner = Player::X;
            break;
        }
        if board.winner != Player::NEITHER {
            break;
        }
        last_move = x_ai.get_move(last_move);
    }
    board.pretty_print();
    println!("{:?}", board.move_history);
    x_ai.cleanup();
    o_ai.cleanup();
    println!("{:?} wins", board.winner);
    //println!("{:?}", v);
}
