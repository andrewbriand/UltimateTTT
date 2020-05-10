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
    //let mut o_ai = HumanPlayer::new(level);
    //let mut x_ai = PipeAI::new("C:/ultimate-tictactoe/target/release/main.exe".to_string(),
                     //          vec!["10".to_string()]);
    //let mut o_ai = PipeAI::new("C:/ultimate-tictactoe/target/release/main.exe".to_string(),
     //                          vec!["12".to_string()]);
    /*let mut x_ai = SimpleSearchAI::new(
         |board: &Board| -> i32 {
              if board.winner == Player::O {
                 return -50000;
              } else if board.winner == Player::X {
                 return 50000;
              }
              return 0;
         }
        , 6);*/
    /*let mut o_ai = SimpleSearchAI::new(
         |board: &Board| -> i32 {
              if board.winner == Player::O {
                 return 50000;
              } else if board.winner == Player::X {
                 return -50000;
              }
              return 0;
         }
        , 7);*/
    let mut o_ai = SimpleSearchAI::new(
         |board: &Board| -> i32 {
              if board.winner == Player::O {
                 return 50000;
              } else if board.winner == Player::X {
                 return -50000;
              }
              let mut result = 0;
              for i in [0, 9, 18, 27, 36, 45, 54, 63, 72].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 1}) {
                        Player::O => result += 500,
                        Player::X => result -= 500,
                        _ => ()
                   }
              }
                  match board.get(Square { top_left: 36,
                                        level: 1}) {
                        Player::O => result += 1000,
                        Player::X => result -= 1000,
                        _ => ()
                   }
              for i in [4, 13, 22, 31, 40, 49, 58, 67, 76].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 0}) {
                        Player::O => result += 50,
                        Player::X => result -= 50,
                        _ => ()
                   }
              }
              return result;
         }
        , 10);
    let mut x_ai = SimpleSearchAI::new(
         |board: &Board| -> i32 {
              if board.winner == Player::O {
                 return -50000;
              } else if board.winner == Player::X {
                 return 50000;
              }
              let mut result = 0;
              for i in [0, 9, 18, 27, 36, 45, 54, 63, 72].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 1}) {
                        Player::O => result -= 500,
                        Player::X => result += 500,
                        _ => ()
                   }
              }
                  match board.get(Square { top_left: 36,
                                        level: 1}) {
                        Player::O => result -= 1000,
                        Player::X => result += 1000,
                        _ => ()
                   }
              for i in [4, 13, 22, 31, 40, 49, 58, 67, 76].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 0}) {
                        Player::O => result -= 50,
                        Player::X => result += 50,
                        _ => ()
                   }
              }
              return result;
         }
        , 10);
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
