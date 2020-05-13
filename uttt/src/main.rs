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
use board::Square;
mod simplesearch;
use simplesearch::SimpleSearchAI;

mod montecarloai;
use montecarloai::MonteCarloAI;
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
    let mut o_ai = SimpleSearchAI::new(
        SimpleSearchAI::ab_then_mc(20)
    , 4);
    let mut x_ai = SimpleSearchAI::new(
        SimpleSearchAI::abriand_eval_1()
    , 4);
    play_game(&mut x_ai, &mut o_ai);
}

fn play_game(x_ai: &mut dyn AI, o_ai: &mut dyn AI) -> Player {
    let mut last_move = x_ai.get_move(-1);
    let mut board = Board::new(2);
    loop {
        if last_move == -1 {
            println!("X forfeited");
            board.winner = Player::O;
            break;
        }
        if !board.make_move(last_move as usize) {
            println!("X made an illegal move {}", last_move);
            board.winner = Player::O;
            break;
        }
        board.pretty_print();
        println!("");
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
            println!("O made an illegal move {}", last_move);
            board.winner = Player::X;
            break;
        }
        board.pretty_print();
        println!("");
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
    return board.winner;
}
