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

use std::collections::HashMap;

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
    let ais: Vec<(String, Box<dyn Fn() -> Box<dyn AI>>)> = 
        vec![
            ("abriand1_10".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::abriand_eval_1(), 10)))
            ),
            ("abriand1_12".to_string(), 
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::abriand_eval_1(), 12)))
            ),
            ("ab_then_mc_10_10".to_string(),
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::ab_then_mc(10), 10)))
            ),
            ("ab_then_mc_10_20".to_string(),
            Box::new(move || Box::new(
                SimpleSearchAI::new(SimpleSearchAI::ab_then_mc(20), 10)))
            ),
            ("geng_10".to_string(),
            Box::new(move || Box::new(
                PipeAI::new("C:/ultimate-tictactoe/target/release/main.exe".to_string(),
                            vec!["10".to_string()])))
            ),
            ("geng_12".to_string(),
            Box::new(move || Box::new(
                PipeAI::new("C:/ultimate-tictactoe/target/release/main.exe".to_string(),
                            vec!["12".to_string()])))
            ),
            ("mcts_100000".to_string(),
            Box::new(move || Box::new(
                MonteCarloAI::new()
            ))),
        ];
    let mut games: HashMap<(String, String), Player> = HashMap::new();
    let mut scores: Vec<f32> = vec![0.0; ais.len()];
    for x_idx in 0..ais.len() {
        for o_idx in 0..ais.len() {
            let (o_name, o_ctor) = &ais[o_idx];
            let (x_name, x_ctor) = &ais[x_idx];
            match play_game(&mut *x_ctor(), &mut *o_ctor()) {
                Player::X => {
                    scores[x_idx] += 1.0;
                    games.insert((x_name.clone(), o_name.clone()), Player::X);
                },
                Player::O => {
                    scores[o_idx] += 1.0;
                    games.insert((x_name.clone(), o_name.clone()), Player::O);
                },
                Player::DEAD => {
                    scores[x_idx] += 0.5;
                    scores[o_idx] += 0.5;
                    games.insert((x_name.clone(), o_name.clone()), Player::DEAD);
                },
                Player::NEITHER => panic!("NEITHER won"),
            };
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
