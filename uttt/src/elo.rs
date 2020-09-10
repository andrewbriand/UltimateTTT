mod ai;
/** Binary target that runs a certain number of matches (or indefinitely) based on the ELO rating
 * system
 */
mod board;
mod pipeai;
mod uti;

use ai::AI;
use board::{Board, Player};
use pipeai::CGPipeAI;

use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, channel, Receiver, Sender};

#[derive(Clone)]
struct Bot {
    name: String,
    rating: i32,
    n_games: u32,
    exe_path: String,
    args: Vec<String>, // not a Vec since I don't want to handle copying (Gary)
}

impl Bot {
    // TODO custom starting rating and n_games
    pub fn new(name: &str, exe_path: &str, args: Vec<&str>) -> Bot {
        return Bot {
            name: name.to_string(),
            rating: 1500,
            n_games: 0,
            exe_path: exe_path.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        };
    }
}

fn from_rand_vec(probs: &Vec<f32>) -> usize {
    let sum: f32 = probs.iter().sum();
    let mut cum_prob = 0.0;
    let mut cum_probs = Vec::new();
    for i in 0..probs.len() {
        cum_prob += probs[i] / sum;
        cum_probs.push(cum_prob);
    }
    assert!((cum_prob - 1.0f32).abs() < 1e-6);

    let r: f32 = thread_rng().gen();
    for i in 0..cum_probs.len() {
        if r < cum_probs[i] {
            return i;
        }
    }
    eprintln!("from_rand_vec() reached end of loop");
    return probs.len();
}

struct SortClosure<'a> {
    bots: &'a Vec<Bot>,
    rating: i32,
}

impl<'a> SortClosure<'a> {
    fn call(&self, indices: &mut Vec<usize>) {
        indices.sort_by_key(|idx| (self.rating - self.bots[*idx].rating).abs());
    }
}

async fn play_game(x_bot: Bot, o_bot: Bot) -> Player {
    // 3 seconds to get ready
    let ready_dur = Duration::from_secs(1);
    let mut x_ai = CGPipeAI::new(x_bot.exe_path.clone(), x_bot.args.clone());
    let mut o_ai = CGPipeAI::new(o_bot.exe_path.clone(), o_bot.args.clone());

    let x_ready = x_ai.ready(ready_dur);
    let o_ready = o_ai.ready(ready_dur);
    let x_ready = x_ready.await;
    let o_ready = o_ready.await;

    if !x_ready && !o_ready {
        //eprintln!("Neither got ready in time. It is drawn.");
        return Player::DEAD;
    }
    if !x_ready {
        //eprintln!("X wasn't able to get ready in time. O wins.");
        return Player::O;
    }
    if !o_ready {
        //eprintln!("O wasn't able to get ready in time. X wins.");
        return Player::X;
    }

    let mut last_move = x_ai
        .get_move(-1, x_ai.get_rem_time(), o_ai.get_rem_time())
        .await;
    let mut board = Board::new(2);
    let mut i = 0;
    loop {
        i += 1;
        if last_move == -1 {
            // println!("X forfeited (or timed out/sent illegal command). TODO see pipai.rs::get_move()");
            board.winner = Player::O;
            break;
        }
        if !board.make_move(last_move as usize) {
            // println!("X made an illegal move {}", last_move);
            board.winner = Player::O;
            break;
        }
        //board.pretty_print();
        // println!("");
        if board.winner != Player::NEITHER {
            break;
        }
        last_move = o_ai
            .get_move(last_move, x_ai.get_rem_time(), o_ai.get_rem_time())
            .await;
        if last_move < 0 {
            bad_last_move(last_move, &o_bot.name);
            board.winner = Player::X;
            break;
        }
        if !board.make_move(last_move as usize) {
            //println!("O made an illegal move {}", last_move);
            board.winner = Player::X;
            break;
        }
        //board.pretty_print();
        //println!("");
        if board.winner != Player::NEITHER {
            break;
        }
        last_move = x_ai
            .get_move(last_move, x_ai.get_rem_time(), o_ai.get_rem_time())
            .await;
        if last_move < 0 {
            bad_last_move(last_move, &x_bot.name);
            board.winner = Player::O;
            break;
        }
    }
    x_ai.cleanup();
    o_ai.cleanup();
    // println!("{:?} wins", board.winner);
    return board.winner;
}

fn bad_last_move(last_move: i64, name: &str) {
    if last_move == -1 {
        eprintln!("{} timed out.", name,);
    } else if last_move == -2 {
        eprintln!("{} sent illegal command", name,);
    }
}

struct GameResult {
    x_idx: usize,
    o_idx: usize,
    winner: Player,
}

struct GamePool {
    bots: Vec<Bot>,
    receivers: Vec<Receiver<GameResult>>,
    max_threads: usize,
}

impl GamePool {
    fn new(bots: Vec<Bot>, max_threads: usize) -> GamePool {
        return GamePool {
            bots: bots,
            receivers: Vec::new(),
            max_threads: max_threads,
        };
    }

    async fn run_game(
        x_bot: Bot,
        o_bot: Bot,
        x_idx: usize,
        o_idx: usize,
        mut tx: Sender<GameResult>,
    ) {
        let winner = play_game(x_bot, o_bot).await;
        if let Err(_) = tx
            .send(GameResult {
                x_idx: x_idx,
                o_idx: o_idx,
                winner: winner,
            })
            .await
        {
            eprintln!("send game result failed");
        }
    }

    async fn bitch() {
        let mut output = File::create("hi.txt").unwrap();
        write!(output, "Rust\n💖\nFun").unwrap();
    }

    async fn enqueue(&mut self, a_idx: usize, b_idx: usize) {
        self.enqueue_helper(a_idx, b_idx).await;
        self.enqueue_helper(b_idx, a_idx).await;
    }

    async fn enqueue_helper(&mut self, a_idx: usize, b_idx: usize) {
        assert_ne!(a_idx, b_idx);
        debug_assert!(self.receivers.len() <= self.max_threads);
        if self.receivers.len() == self.max_threads {
            // wait for opening
            /*
            loop {
                for i in 0..self.receivers.len() {
                    match self.receivers[i].try_recv() {
                        Ok(r) => {
                            // TODO handle result
                            self.update_ratings(r);
                            break;
                        }
                        Err(e) => {} // sender hasn't sent message
                    }
                    self.receivers.remove(i);
                    break;
                }
                // TODO maybe sleep for a little bit?
            }
            */
            if let Some(r) = self.receivers[0].recv().await {
                let res_str = match r.winner {
                    Player::X => "1-0",
                    Player::O => "0-1",
                    Player::DEAD => "0.5-0.5",
                    Player::NEITHER => panic!("Player::NEITHER should not occur here"),
                };
                let (x_diff, o_diff) = self.update_ratings(&r);
                println!(
                    "Ended {} v. {}: {}. X={} ({:+}), O={} ({:+})",
                    self.bots[r.x_idx].name, self.bots[r.o_idx].name, res_str,
                    self.bots[r.x_idx].rating, x_diff, self.bots[r.o_idx].rating, o_diff,
                );

                self.receivers.remove(0);
            } else {
                eprintln!("received None");
            }
        } else {
            let (tx, rx) = channel(1024); // TODO buffer
            let mut tx = tx.clone();
            let x_bot = self.bots[a_idx].clone();
            let o_bot = self.bots[b_idx].clone();
            //let result = GamePool::run_game(x_bot, o_bot, a_idx, b_idx, tx);
            //let result = GamePool::bitch();
            tokio::spawn(async move {
                println!("Starting {} v. {}...", x_bot.name, o_bot.name);
                let winner = play_game(x_bot, o_bot).await;
                if let Err(_) = tx
                    .send(GameResult {
                        x_idx: a_idx,
                        o_idx: b_idx,
                        winner: winner,
                    })
                    .await
                {
                    eprintln!("send game result failed");
                }
            });
            self.receivers.push(rx);
        }
    }

    async fn join_all(&mut self) {
        while self.receivers.len() != 0 {
            let mut rec = self.receivers.pop().unwrap();
            let result = rec.recv().await.unwrap();
            self.update_ratings(&result);
        }
    }

    fn update_ratings(&mut self, result: &GameResult) -> (i32, i32) {
        // ratings of X and O
        let r_x = self.bots[result.x_idx].rating;
        let r_o = self.bots[result.o_idx].rating;
        // expected game results
        let e_x = GamePool::expected_score(r_x, r_o);
        let e_o = GamePool::expected_score(r_o, r_x);
        let k_x = 800.0 / (self.bots[result.x_idx].n_games as f32 + 20.0);
        let k_o = 800.0 / (self.bots[result.o_idx].n_games as f32 + 20.0);
        // actual game results
        let a_x;
        let a_o;

        match result.winner {
            Player::X => {
                a_x = 1.0;
                a_o = 0.0;
            }
            Player::O => {
                a_x = 0.0;
                a_o = 1.0;
            }
            Player::DEAD => {
                a_x = 0.5;
                a_o = 0.5;
            }
            Player::NEITHER => panic!("Winner is NEITHER"),
        }
        let x_rating = self.bots[result.x_idx].rating;
        let x_diff = (k_x * (e_x - a_x)) as i32; // TODO floor at some value?
        self.bots[result.x_idx].rating = x_rating + x_diff;
        let o_rating = self.bots[result.o_idx].rating;
        let o_diff = (k_o * (e_o - a_o)) as i32;
        self.bots[result.o_idx].rating = o_rating + o_diff;
        self.bots[result.x_idx].n_games += 1;
        self.bots[result.o_idx].n_games += 1;
        return (x_diff, o_diff);
    }

    fn expected_score(r_me: i32, r_them: i32) -> f32 {
        return 1.0 / (1.0 + 10.0f32.powf((r_them as f32 - r_me as f32) / 400.0));
    }
}

#[tokio::main(max_threads = 8)]
async fn main() {
    // number of double-matches, i.e. including switch-sides. There will be total 2x matches.

    let N_GAMES = 100;  // total number of games to run
    let K_CLOSEST = 3;  // match a bot with its K nearest neighbors in ELO every time
    let N_GAMES_PER_UPDATE = 24;  // print ranking after at least this many games
    let N_THREADS = 6;  // number of games run concurrently

    let gary_path = "C:/Users/Gary/Code/uttt/target/release/codingame.exe";
    let mut c_args = vec![];
    let mut c = 0.6f32;
    while c <= 1.6f32 {
        c_args.push(c);
        c += 0.05;
    }
    let bots: Vec<Bot> = c_args
        .iter()
        .map(|c| Bot::new(&format!("gary c={}", c), gary_path, vec![&c.to_string()]))
        .collect();
    let n_bots = bots.len();
    assert!(bots.len() >= 2);
    let mut pool = GamePool::new(bots, N_THREADS);
    let mut indices = Vec::new(); // intermediate vector for getting K closest bots to some bot
    for i in 0..n_bots {
        indices.push(i);
    }

    let mut n_played = 0;
    let mut last_n_played = 0;
    loop {
        let mut min_i = 0;
        let mut min_n_games = N_GAMES + 10;
        for i in 0..n_bots {
            if pool.bots[i].n_games < min_n_games {
                min_i = i;
                min_n_games = pool.bots[i].n_games;
            }
        }
        // TODO lock
        let sort_c = SortClosure {
            bots: &pool.bots,
            rating: pool.bots[min_i].rating,
        };
        sort_c.call(&mut indices);
        let mut i = 0;
        while i < K_CLOSEST.min(n_bots) {
            if min_i == indices[i] {
                i += 1;
                continue;
            }
            pool.enqueue(min_i, indices[i]).await;
            n_played += 2;
            i += 1;
        }

        if n_played - last_n_played >= N_GAMES_PER_UPDATE {
            last_n_played = n_played;
            println!("------RANKING UPDATE------");
            let mut bots = pool.bots.clone();
            bots.sort_by_key(|b| std::cmp::Reverse(b.rating));
            for bot in &bots {
                println!(
                    "'{}'; Elo {}; Command '{}'; Args '{}'; Games played {}",
                    bot.name,
                    bot.rating,
                    bot.exe_path,
                    bot.args.join(", "),
                    bot.n_games
                );
            }
        }
        if n_played >= N_GAMES {
            break;
        }
    }
    pool.join_all().await; // finish up
    println!("-----FINAL RANKINGS-----");
    let mut bots = pool.bots.clone();
    bots.sort_by_key(|b| std::cmp::Reverse(b.rating));
    for bot in &bots {
        println!(
            "'{}'; Elo {}; Command '{}'; Args '{}'; Games played {}",
            bot.name,
            bot.rating,
            bot.exe_path,
            bot.args.join(", "),
            bot.n_games
        );
    }
}
