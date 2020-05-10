use crate::ai::AI;
use crate::board::Board;

pub struct SimpleSearchAI {
    board: Board,
    eval: fn(&Board) -> i32,
    depth: usize,
}

impl AI for SimpleSearchAI {

    fn get_move(&mut self, last_move: i64) -> i64 {
        if last_move != -1 {
            self.board.make_move(last_move as usize);
        }
        let mut alpha = -100000000;
        let beta = 100000000;
        let mut result_move : i64 = -1;
        let moves = self.board.get_moves();
        for m in moves {
            let mut new_board = self.board.clone();
            new_board.make_move(m);
            let score = -self.search(&new_board, self.depth - 1, -beta, -alpha);
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
    pub fn new(_eval: fn(&Board) -> i32, _depth: usize) 
        -> SimpleSearchAI {
        SimpleSearchAI {
            board: Board::new(2),
            eval: _eval,
            depth: _depth,
        }
    }

    pub fn search(&self, board: &Board, depth: usize, 
                  _alpha: i32, beta: i32) -> i32 {
        let mut alpha = _alpha;
        if depth == 0 {
            return (self.eval)(board);
        }
        let moves = board.get_moves();
        if moves.len() == 0 {
            if depth % 2 == 0 {
                return (self.eval)(board);
            } else {
                return -(self.eval)(board);
            }
        }
        for m in moves {
            let mut new_board = board.clone();
            new_board.make_move(m);
            let score = -self.search(&new_board, depth - 1, -beta, -alpha);
            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                return alpha;
            }
        }
        return alpha;
    }
}