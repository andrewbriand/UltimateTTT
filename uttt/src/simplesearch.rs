use crate::ai::AI;
use crate::board::Board;
use crate::board::Player;
use crate::board::Square;

pub struct SimpleSearchAI {
    board: Board,
    eval: Box<dyn Fn(&mut Board, Player) -> i32>,
    depth: usize,
    me: Player,
}

impl AI for SimpleSearchAI {

    fn get_move(&mut self, last_move: i64) -> i64 {
        if last_move != -1 {
            self.board.make_move(last_move as usize);
        }
        self.me = self.board.get_to_move();
        let mut alpha = -100000000;
        let beta = 100000000;
        let mut result_move : i64 = -1;
        let moves = self.board.get_moves();
        let mut new_board = self.board.clone();
        for m in moves {
            new_board.make_move(m);
            let score = -self.search(&mut new_board, self.depth - 1, -beta, -alpha);
            new_board.undo_move();
            if score > alpha {
                alpha = score;
                result_move = m as i64;
            }

            if alpha >= beta {
                self.board.make_move(result_move as usize);
                println!("result_score: {}", alpha);
                return result_move;
            }
        }
        self.board.make_move(result_move as usize);
        println!("result_score: {}", alpha);
        return result_move;
    }

    fn cleanup(&mut self) {}
}

impl SimpleSearchAI {
    pub fn new<'a>(_eval: Box<dyn Fn(&mut Board, Player) -> i32>, _depth: usize) 
        -> SimpleSearchAI {
        SimpleSearchAI {
            board: Board::new(2),
            eval: _eval,
            depth: _depth,
            me: Player::NEITHER,
        }
    }

    pub fn search(&self, board: &mut Board, depth: usize, 
                  _alpha: i32, beta: i32) -> i32 {
        let mut alpha = _alpha;
        if depth == 0 {
            return (self.eval)(board, self.me);
        }
        let moves = board.get_moves();
        if moves.len() == 0 {
            if depth % 2 == 0 {
                return (self.eval)(board, self.me);
            } else {
                return -(self.eval)(board, self.me);
            }
        }
        for m in moves {
            board.make_move(m);
            let score = -self.search(board, depth - 1, -beta, -alpha);
            board.undo_move();
            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                return alpha;
            }
        }
        return alpha;
    }

    pub fn ab_then_mc(games: usize) -> Box<dyn Fn(&mut Board, Player) -> i32> {
        Box::new(move |_board: &mut Board, me: Player| -> i32 {
              let opponent = match me {
                  Player::X => Player::O,
                  Player::O => Player::X,
                  _ => panic!("AI is not a player"),
              };
              if _board.winner == me {
                 return 50000;
              } else if _board.winner == opponent {
                 return -50000;
              }
            let mut result = 0;
            for _i in 0..games {
                let mut board = _board.clone();
                while board.winner == Player::NEITHER {
                    let moves = board.get_moves();
                    let next_move = moves[rand::random::<usize>() % moves.len()];
                    board.make_move(next_move);
                }
                if board.winner == me {
                    result += 1;
                } else if board.winner == opponent {
                    result += -1;
                }
            }
            return result;
        })
    }

    pub fn abriand_eval_1() -> Box<dyn Fn(&mut Board, Player) -> i32> {
        Box::new(move |board: &mut Board, me: Player| -> i32 {
              let opponent = match me {
                  Player::X => Player::O,
                  Player::O => Player::X,
                  _ => panic!("AI is not a player"),
              };
              if board.winner == me {
                 return 50000;
              } else if board.winner == opponent {
                 return -50000;
              }
              let mut result = 0;
              for i in [0, 9, 18, 27, 36, 45, 54, 63, 72].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 1}) {
                        x if me == x => result += 500,
                        x if opponent == x => result -= 500,
                        _ => ()
                   }
              }
                  match board.get(Square { top_left: 36,
                                        level: 1}) {
                        x if me == x => result += 1000,
                        x if opponent == x => result -= 1000,
                        _ => ()
                   }
              for i in [4, 13, 22, 31, 40, 49, 58, 67, 76].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 0}) {
                        x if me == x => result += 50,
                        x if opponent == x => result -= 50,
                        _ => ()
                   }
              }
              return result;
        })
    }
    pub fn diagonal() -> Box<dyn Fn(&mut Board, Player) -> i32> {
        Box::new(move |board: &mut Board, me: Player| -> i32 {
              let opponent = match me {
                  Player::X => Player::O,
                  Player::O => Player::X,
                  _ => panic!("AI is not a player"),
              };
              if board.winner == me {
                 return 50000;
              } else if board.winner == opponent {
                 return -50000;
              }
              let mut result = 0;
                  match board.get(Square { top_left: 36,
                                        level: 1}) {
                        x if me == x => result += 1000,
                        x if opponent == x => result -= 1000,
                        _ => ()
                   }
              for i in [0, 36, 72].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 1}) {
                        x if me == x => result += 1000,
                        x if opponent == x => result -= 1000,
                        _ => ()
                   }
              }
              return result;
        })
    }
}