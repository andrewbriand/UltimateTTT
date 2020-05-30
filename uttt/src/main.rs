#![feature(llvm_asm)]
//use structopt::StructOpt;
mod board;
mod bitboard;
//use bitboard::BitBoard;
mod killermovesearch;
//use killermovesearch::KillerMoveAI;

mod ai;
use ai::AI;
pub use board::Board;
mod humanplayer;
mod quietextend;
use quietextend::QuietExtendAI;
mod pipeai;
pub use pipeai::PipeAI;
pub use humanplayer::HumanPlayer;
use board::Player;
mod simplesearchcenter;
use simplesearchcenter::SimpleSearchCenterAI;
mod simplesearch;
use simplesearch::SimpleSearchAI;
 use std::time::Instant;

use std::collections::HashMap;

mod montecarloai;
use montecarloai::MonteCarloAI;
//use text_io::read;
//use std::time::Instant;

/*#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    o_ai_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    x_ai_path: std::path::PathBuf,
}*/

fn main() {
    let ais: Vec<(String, Box<dyn Fn() -> Box<dyn AI>>)> = 
        vec![
            /*("javascript_10".to_string(),
            Box::new(move || Box::new(
                PipeAI::new("C:/Program Files/nodejs/node.exe".to_string(), 
                vec!["uttt.js".to_string(), "10".to_string()])
            ))
            ),*/
            /*("diagonal_10".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::diagonal(), 10)))
            ),*/
            ("abriand_10_center".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchCenterAI::new(SimpleSearchAI::diagonal2(), 10)))
            ),
            ("abriand_10".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::diagonal2(), 10)))
            ),
            ("centers_10".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::centers(), 10)))
            ),
            ("ggeng_10".to_string(),
            Box::new(move || Box::new(
                PipeAI::new("C:/ultimate-tictactoe/target/release/main.exe".to_string(),
                      vec!["10".to_string()])))
            ),
            ("good_extend_10".to_string(), 
            Box::new(move || Box::new(
                QuietExtendAI::new(SimpleSearchAI::diagonal2(), 10)))
            ),

            /*("killer_10".to_string(), 
            Box::new(move || Box::new(
                KillerMoveAI::new(KillerMoveAI::abriand_eval_1(), 10)))
            ),
            ("quiet_10".to_string(), 
            Box::new(move || Box::new(
                QuietExtendAI::new(QuietExtendAI::abriand_eval_1(), 10)))
            ),*/
            /*("abriand1_14".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::abriand_eval_1(), 14)))
            ),*/
        ];
    let mut games: HashMap<(String, String), Player> = HashMap::new();
    let mut scores: Vec<f32> = vec![0.0; ais.len()];
    for _i in 0..6 {
    for x_idx in 0..ais.len() {
        for o_idx in 0..ais.len() {
            if x_idx != o_idx {
                let (o_name, o_ctor) = &ais[o_idx];
                let (x_name, x_ctor) = &ais[x_idx];
                match play_game(&mut *x_ctor(), &mut *o_ctor()) {
                    Player::X => {
                        scores[x_idx] += 1.0;
                        games.insert((x_name.clone(), o_name.clone() + " " + &_i.to_string()), Player::X);
                    },
                    Player::O => {
                        scores[o_idx] += 1.0;
                        games.insert((x_name.clone(), o_name.clone() + " " + &_i.to_string()), Player::O);
                    },
                    Player::DEAD => {
                        scores[x_idx] += 0.5;
                        scores[o_idx] += 0.5;
                        games.insert((x_name.clone(), o_name.clone() + " " + &_i.to_string()), Player::DEAD);
                    },
                    Player::NEITHER => panic!("NEITHER won"),
                };
            }
        }
    }
    }
    for g in games {
        println!("{} vs {}: {:?}", (g.0).0, (g.0).1, g.1);
    }
    println!("");
    for s_idx in 0..scores.len() {
        println!("{}: {}", ais[s_idx].0, scores[s_idx]);
    }
}

fn play_game(x_ai: &mut dyn AI, o_ai: &mut dyn AI) -> Player {
    let mut times_vec = Vec::new();
    let mut now = Instant::now();
    let mut last_move = x_ai.get_move(-1);
    times_vec.push(now.elapsed().as_millis());
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
        now = Instant::now();
        last_move = x_ai.get_move(last_move);
        times_vec.push(now.elapsed().as_millis());
    }
    board.pretty_print();
    println!("{:?}", board.move_history);
    println!("{:?}", times_vec);
    x_ai.cleanup();
    o_ai.cleanup();
    println!("{:?} wins", board.winner);
    return board.winner;
}
