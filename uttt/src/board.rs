use std::collections::HashMap;
//use std::thread;
use std::hash::{Hash};
use std::time::Instant;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Player {
    X,
    O,
    // Neither player occupies this square
    NEITHER,
    // Neither player occupies this square,
    // but it can never be occupied because it is
    // in a higher occupied square or is drawn
    DEAD,
}


// Definitions:
// Space - the smallest unit of the board, where a player can place
//         an X or O
// Square - A square is either:
//              1. A single space
//              2. A collection of 9 squares
// Drawn - A square is drawn if all of its subsquares are occupied

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Square {
    // the integer corresponding to the 
    // space in the top left corner of this square
    pub top_left: usize,
    // level 0 is an individual space,
    // level 1 is a 3x3 board, 
    // level 2 is a 9x9 board, etc
    pub level: usize,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Turn {
    pub space: usize,
    pub capture: usize,
    pub bounds: Square,
}

// Each tile of the tic tac toe board is assigned an integer
// max_level = 1:
// 0 1 2
// 3 4 5
// 6 7 8
// max_level = 2:
// 00 01 02  09 10 11  18 19 20
// 03 04 05  12 13 14  21 22 23
// 06 07 08  15 16 17  24 25 26
//
// 27 28 29  36 37 38  45 46 47
// 30 31 32  39 40 41  48 49 50 
// 33 34 35  42 43 44  51 52 53
//
// 54 55 56  63 64 65  72 73 74
// 57 58 59  66 67 68  75 76 77
// 60 61 62  69 70 71  78 79 80
// In the above example, (space: 0, level 1) is the square with its
// top left corner at 00 and its bottom right corner at 08
#[derive(Debug)]
#[derive(Clone)]
pub struct Board {
    // the index of the top level in the board e.g.
    // max_level = 1 is a standard 3x3 tic-tac-toe board
    // max_level = 2 is a 9x9 tic-tac-toe board
    max_level: usize,
    // The player who will make the next move
    to_move: Player,
    // Spaces and their occupation status
    spaces: Vec<Player>,
    // Maps higher level squares to their occupation status
    occupied: HashMap<Square, Player>,
    // Tuple describing the upper left corner and level
    // of the next legal move space
    pub next_legal: Square,
    // The player that has won the game, or NEITHER if
    // the game is still ongoing
    // winner is DEAD if the game is drawn
    pub winner: Player,
    // The moves that have been made up until this point
    // where move_history[move_history.len() - 1] is the last
    // move made
    pub move_history: Vec<Turn>,
    // The size in spaces of a square at level index <= max_level
    level_sizes: Vec<usize>,
}
// Win table for all 3x3 boards
// (Geng, 2020)
static WIN_TABLE: [u64; 8] = [
    0xff80808080808080,
    0xfff0aa80faf0aa80,
    0xffcc8080cccc8080,
    0xfffcaa80fefcaa80,
    0xfffaf0f0aaaa8080,
    0xfffafaf0fafaaa80,
    0xfffef0f0eeee8080,
    0xffffffffffffffff,
];

impl Board {
    pub fn get_to_move(&self) -> Player {
        return self.to_move;
    }
    // Creates a new board with max level max_level_
    // where 1 is a standard 3x3 tic-tac-toe board,
    // 2 is a 9x9 board, etc.
    pub fn new(max_level_: usize) -> Board {
        if max_level_ < 1 {
            panic!("max_level_ must be >= 1");
        }
        let size_ = (3 as usize).pow(max_level_ as u32);
        let mut result = Board {
                to_move: Player::X, // X goes first
                occupied: HashMap::new(),
                spaces: Vec::with_capacity(81),
                // first move can be anywhere
                next_legal: Square { top_left: 0, level: max_level_},
                max_level: max_level_,
                winner: Player::NEITHER,
                move_history: Vec::with_capacity(81),
                level_sizes: Vec::new(),
                //capture_history: Vec::new() 
            };
        // TODO: it might be cleaner to initialize all squares (including
        // higher level ones) with NEITHER
        for _i in 0..(size_*size_) {
            result.spaces.push(Player::NEITHER);
        }
        for _i in 0..9 {
            result.spaces.push(Player::NEITHER);
        }
        result.spaces.push(Player::NEITHER);
        for i in 0..=result.max_level+2 {
            result.level_sizes.push((3 as usize).pow(2*i as u32));
        }
        return result;
    }

   // 00 01 02  09 10 11  18 19 20
   // 03 04 05  12 13 14  21 22 23
   // 06 07 08  15 16 17  24 25 26
   //
   // 27 28 29  36 37 38  45 46 47
   // 30 31 32  39 40 41  48 49 50 
   // 33 34 35  42 43 44  51 52 53
   //
   // 54 55 56  63 64 65  72 73 74
   // 57 58 59  66 67 68  75 76 77
   // 60 61 62  69 70 71  78 79 80
    pub fn pretty_print(&self) {
        // TODO: generalize this to n-levels
        // print rows in order
        for y in [0 as i64, 3, 6, -1, 27, 30, 33, -1, 54, 57, 60].iter() {
            if *y == -1 {
                println!("----------------------");
            } else {
                for x in [0 as i64, 1, 2, -1, 9, 10, 11, -1, 18, 19, 20].iter() {
                    if *x == -1 {
                        print!("| ");
                    } else {
                        let i = *y + *x;
                        match self.spaces[i as usize] {
                            Player::X => print!("X "),
                            Player::O => print!("O "),
                            Player::NEITHER => print!("- "),
                            Player::DEAD => print!("+ "),
                        }
                    }
                }
                println!("");
            }
        }
    }

    pub fn get(&self, sqr: Square) -> Player {
        if sqr.level == 0 {
            return self.spaces[sqr.top_left];
        }
        if sqr.level == 1 {
            return self.spaces[81 + sqr.top_left / 9];
        }
        if sqr.level == 2 {
            return self.spaces[90];
        }
        panic!("Call to get with sqr > max_level: {}", sqr.level);
    }

    fn set(&mut self, sqr: Square, player: Player) {
        if sqr.level == 0 {
            self.spaces[sqr.top_left] = player;
        } else if sqr.level == 1 {
            self.spaces[81 + sqr.top_left / 9] = player;
        } else if sqr.level == 2 {
            self.spaces[90] = player;
        }
    }

    // Return the integer corresponding to the bottom
    // right space of sqr
    fn bottom_right(&self, sqr: Square) -> usize {
        if sqr.level == 1 {
            return sqr.top_left + 8;
        } else if sqr.level == 2 {
            return 80;
        }
        return sqr.top_left + self.level_sizes[sqr.level] - 1;
    }

    // Is the given space in the move bounds for this turn?
    fn in_bounds(&self, space: usize) -> bool {
       //println!("{}", space);
       space >= self.next_legal.top_left && 
       space <= self.bottom_right(self.next_legal)
    }

    // Mark any spaces marked NEITHER in sqr as DEAD
    /*fn mark_as_dead(&mut self, sqr: &Square) {
        for i in sqr.top_left..=self.bottom_right(*sqr) {
            if self.spaces[i] == Player::NEITHER {
                self.spaces[i] = Player::DEAD;
            }
        }
    }*/

    // Mark any spaces marked DEAD in sqr as NEITHER
    /*fn mark_as_neither(&mut self, sqr: &Square) {
        for i in sqr.top_left..=self.bottom_right(*sqr) {
            if self.spaces[i] == Player::DEAD {
                self.spaces[i] = Player::NEITHER;
            }
        }
    }*/

    // Push all spaces that are marked NEITHER in sqr
    // to vec
    /*fn get_open_spaces(&self, sqr: Square, vec: &mut Vec<usize>) {
        for i in sqr.top_left..=self.bottom_right(sqr) {
            if self.spaces[i] == Player::NEITHER {
                vec.push(i);
            }
        }
    }*/

    // Returns a vector of the current legal moves
    pub fn get_moves(&self) -> Vec<usize> {
        let mut vec = Vec::with_capacity(81);
        for i in self.next_legal.top_left..=self.bottom_right(self.next_legal) {
            if self.spaces[i] == Player::NEITHER 
               && self.get(self.ascend(&Square{level: 0, 
                   top_left: i}).0) == Player::NEITHER {
                vec.push(i);
            }
        }
        //self.get_open_spaces(self.next_legal, &mut vec);
        return vec;
    }

    // Update the bounds for the next move
    // to be as if move_sqr was the last move made
    fn update_move_bounds(&mut self, move_sqr: &Square) {
        // Update the bounds for the next move
        // Ascend from the space the move was made in
        // and save which subsquare it was
        let (mid_square, i) = self.ascend(move_sqr);
        // Ascend again, and then descend into our next
        // legal move space using the saved subsquare number
        let (highest_sqr, _) = self.ascend(&mid_square);
        self.next_legal = self.descend(&highest_sqr, i);
        // If the calculated move space is occupied, ascend the legal
        // move space until it is not
        //
        // Note: this assumes unoccupied squares (which
        // could reasonably be marked NEITHER but aren't
        // currently) in levels
        // higher than 0 are not in the occupied map
        //
        // We know that this will not result in a next_legal
        // larger than the entire board, because we have already
        // determined that the board is not drawn or won
        while self.get(self.next_legal) != Player::NEITHER  {
            if self.next_legal.level >= 2 {
                println!("Wrong: {}", self.next_legal.level);
            }
            let (temp, _) = self.ascend(&self.next_legal);
            self.next_legal = temp;
        }
    }

    // Switch to_move to the next player
    fn next_player(&mut self) {
        if self.to_move == Player::X {
            self.to_move = Player::O;
        } else {
            self.to_move = Player::X;
        }
    }

    // make the next move on space space
    // returns true iff the move is legal
    // does not affect board state if the move is illegal
    pub fn make_move(&mut self, space: usize) -> bool {
        let move_sqr = Square {top_left: space, level: 0};
        // Make sure this square is available
        if self.spaces[space] != Player::NEITHER
            || self.get(self.ascend(&move_sqr).0) != Player::NEITHER
            || self.winner != Player::NEITHER {
            return false;
        }
        // Check if the move is in the legal bounds
        if !self.in_bounds(space) {
            return false;
        }
        // Write this move to the board
        self.spaces[space] = self.to_move;
        // Put the move into the move history
        //self.move_history.push(space);
        let mut turn = Turn {
            bounds: self.next_legal,
            capture: 81,
            space: space,
        };
        
        // Update occupied
        let (mut _check_sqr, _) = self.ascend(&move_sqr);
        let check_sqr = &mut _check_sqr;
        // Check levels for captures
        while check_sqr.level <= self.max_level {
            let victorious_player = self.check_victory(&check_sqr);
            if victorious_player != Player::NEITHER {
                // This player or DEAD now occupies this square
                //self.occupied.insert(*check_sqr, victorious_player);
                self.set(*check_sqr, victorious_player);
                //self.mark_as_dead(check_sqr);
                if check_sqr.level == 1 {
                    turn.capture = check_sqr.top_left;
                }
                // If this is the top level, the capturing player
                // wins the game, or the game is drawn (winner = DEAD)
                if check_sqr.level == self.max_level {
                    self.winner = victorious_player;
                }
            } else {
                // If no capture or draw happened at this level,
                // then none can happen at any higher levels
                break;
            }
            let (_check_sqr, _) = self.ascend(check_sqr);
            *check_sqr = _check_sqr;
        }
        if self.winner == Player::NEITHER {
            self.update_move_bounds(&move_sqr);
        }
        self.move_history.push(turn);
        self.next_player();
        return true;
    }

    // Undo the most recent move unless no moves have been made
    // in which case does nothing
    // Returns false iff no moves have been made
    pub fn undo_move(&mut self) -> bool {
        if self.move_history.len() == 0 {
            return false;
        }
        self.winner = Player::NEITHER;
        self.set(Square{top_left: 0, level: 2}, Player::NEITHER);
        let t = self.move_history.pop().unwrap();
        self.set(Square {level: 0, top_left: t.space},
                 Player::NEITHER);
        if t.capture != 81 {
            self.set(Square {level: 1, top_left: t.capture},
                    Player::NEITHER);
        }
        self.next_legal = t.bounds;
        self.next_player();
        return true;
    }

   // Return one of the nine sub-squares that make up sqr
   // where i is one of
   // 0 1 2
   // 3 4 5
   // 6 7 8
   // For example in the two-level board:
   // 00 01 02  09 10 11  18 19 20
   // 03 04 05  12 13 14  21 22 23
   // 06 07 08  15 16 17  24 25 26
   //
   // 27 28 29  36 37 38  45 46 47
   // 30 31 32  39 40 41  48 49 50 
   // 33 34 35  42 43 44  51 52 53
   //
   // 54 55 56  63 64 65  72 73 74
   // 57 58 59  66 67 68  75 76 77
   // 60 61 62  69 70 71  78 79 80
   // Descend((54, 1), 2) gives (56, 0)
   // Descend((0, 2), 8) gives (72, 1)
   fn descend(&self, sqr: &Square, i: usize) -> Square {
        if sqr.level == 1 {
            return Square {top_left: sqr.top_left + i, level: 0};
        } else if sqr.level == 2 {
            return Square {top_left: i * 9, level: 1};
        }
        Square { top_left: sqr.top_left + 
                      i * self.level_sizes[sqr.level - 1], 
                level: sqr.level - 1}
   } 

   // Return the square that contains sub-square sqr
   // and which subsquare it was, where sub-squares are numbered
   // as follows:
   // 0 1 2
   // 3 4 5
   // 6 7 8
   fn ascend(&self, sqr: &Square) -> (Square, usize) {
       if sqr.level == 0 {
           let i = sqr.top_left % 9;
           return (Square {top_left: sqr.top_left - i, level: 1}, i);
       } else if sqr.level == 1 {
           return (Square {top_left: 0, level: 2}, sqr.top_left / 9);
       }
       let f = sqr.top_left % self.level_sizes[sqr.level + 1];
       let i = f / self.level_sizes[sqr.level];
       let tl = sqr.top_left - i * self.level_sizes[sqr.level];
       (Square {top_left: tl, level: sqr.level + 1}, i)
   }

    // Determine if the square with space at its top left corner at level
    // where 0 is the lowest level (i.e. individual squares) has been 
    // won by a player
    // Returns the winner if so, returns NEITHER if no player has won
    // and returns DEAD if the square is drawn (i.e all of its
    // subsquares are occupied)
    // TODO: only check for the last player that moved
    pub fn check_victory(&self, sqr: &Square) -> Player {
        let mut block_x = 0;
        for i in 0..9 {
            if self.get(self.descend(sqr, i)) == self.to_move {
                block_x |= 1 << i;
            }
        }
        if WIN_TABLE[block_x as usize / 64] & (1 << (block_x % 64)) != 0 {
            return self.to_move;
        }
        // Check for draw
        let mut draw = true;
        for i in 0..9 {
            match self.get(self.descend(sqr, i)) {
                Player::NEITHER => {draw = false; break; },
                _ => continue, 
            }
        }

        if draw {
            return Player::DEAD;
        }
        return Player::NEITHER;
    }

    // Determine if the square with space at its top left corner at level
    // where 0 is the lowest level (i.e. individual squares) has been 
    // won by a player
    // Returns the winner if so, returns NEITHER if no player has won
    // and returns DEAD if the square is drawn (i.e all of its
    // subsquares are occupied)
    pub fn check_victory_old(&self, sqr: &Square) -> Player {
        let mut this_board: Vec<Player> = Vec::with_capacity(9); 
        // Put the owners of the 9 subsquares
        // composing sqr into this_board
        let mut counter = 0;
        this_board.resize_with(9, || {
            let move_sqr = self.descend(sqr, counter);
            counter += 1;
            self.get(move_sqr)
        });
        
        // Check the horizontals
        for j in [0, 3, 6].iter() {
            let r = *j;
            if this_board[r] == this_board[r + 1] &&
               this_board[r + 1] == this_board[r + 2] {
                if this_board[r] != Player::NEITHER {
                    return this_board[r];
                }
            }
        }

        // Check the verticals
        for j in [0, 1, 2].iter() {
            let r = *j;
            if this_board[r] == this_board[r + 3] &&
               this_board[r + 3] == this_board[r + 6] {
                if this_board[r] != Player::NEITHER {
                    return this_board[r];
                }
            }
        }

        // Check the diagonals
        if this_board[0] == this_board[4] &&
           this_board[4] == this_board[8] {
            if this_board[0] != Player::NEITHER {
                return this_board[0];
            }
        }

        if this_board[2] == this_board[4] &&
           this_board[4] == this_board[6] {
            if this_board[2] != Player::NEITHER {
               return this_board[2];
            }
        }

        // Check for draw
        let mut draw = true;
        for i in 0..9 {
            if this_board[i] == Player::NEITHER {
                draw = false;
                break;
            }
        }

        if draw {
            return Player::DEAD;
        }
        return Player::NEITHER;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
     fn test_basic_moves_2lv() {
         let mut b = Board::new(2);
         let moves = vec![20, 22, 38, 21, 29, 23, 50, 49, 41, 46, 14, 52];
         for i in moves {
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
         assert!(b.make_move(68));
         assert!(!b.make_move(48));
     }

     #[test]
     fn test_basic_victory_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for i in moves {
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
         assert!(b.winner == Player::O);
     }
     #[test]
     fn test_undo_basic_victory_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for i in &moves {
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", i);
         }
         for _i in &moves {
             b.undo_move();
         }
         for i in &moves {
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", i);
         }
         assert!(b.winner == Player::O);
     }

     #[test]
     fn test_full_square_ascend_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 1, 10, 9, 5, 45, 7, 70, 71, 80, 72, 4, 36, 8, 
                          73, 11, 18, 2, 20, 21, 27, 3, 33, 54, 6, 
                          61, 63, 13];
         for i in moves {
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
     }

     #[test]
     fn test_draw_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 1, 9, 4, 36, 7, 70, 71, 79, 67, 43, 63, 20, 21, 
                          31, 40, 37, 13, 38, 23, 49, 22, 10, 14, 52, 55, 11, 
                          50, 46, 30, 29, 27, 32, 33, 58, 78, 59, 72, 57, 73, 74, 
                          76, 77, 80];
         for i in moves {
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
         assert!(b.winner == Player::DEAD);
     }

     #[test]
     fn test_basic_undo_2lv() {
         let mut b = Board::new(2);
         let moves = vec![20, 22, 38, 21, 29, 23, 50, 49, 41, 46, 14, 52];
         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.make_move(68));
         assert!(!b.make_move(48));

         for _i in 0..(moves.len()+1) {
             assert!(b.undo_move());
         }

         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.make_move(68));
         assert!(!b.make_move(48));
     }

     #[test]
     fn test_basic_victory_undo_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.winner == Player::O);
         for _i in 0..moves.len() {
             assert!(b.undo_move());
         }

         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.winner == Player::O);
     }

     #[test]
     fn test_full_square_ascend_undo_2lv() {
         let mut b = Board::new(2);
         let moves = vec![0, 1, 10, 9, 5, 45, 7, 70, 71, 80, 72, 4, 36, 8, 
                          73, 11, 18, 2, 20, 21, 27, 3, 33, 54, 6, 
                          61, 63, 13];
         for i in moves {
             assert!(b.make_move(i));
             assert!(b.undo_move());
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
     }

     fn get_moves_at_depths(b: &mut Board, depth: usize, out: &mut Vec<usize>) {
         if depth == 0 {
             return;
         }
         let moves = b.get_moves();
         let out_len = out.len();
         out[out_len - depth] += moves.len();
         for m in moves {
             let mut next_b = b.clone();
             assert!(next_b.make_move(m));
             get_moves_at_depths(&mut next_b, depth - 1, out);
         }
     }

     fn get_moves_at_depths_no_vector(b: &mut Board, depth: usize) -> usize {
         if depth == 0 {
             return 1;
         }
         let moves = b.get_moves();
         let mut sum = 0;
         for m in moves {
             let mut next_b = b.clone();
             assert!(next_b.make_move(m));
             sum += get_moves_at_depths_no_vector(&mut next_b, depth - 1);
         }
         return sum;
     }

     fn get_moves_at_depths_undo(b: &mut Board, depth: usize, out: &mut Vec<usize>) {
         let moves = b.get_moves();
         let out_len = out.len();
         out[out_len - depth] += moves.len();
         for m in &moves {
             let temp = out[out_len - 1];
             if !b.make_move(*m) {
                 b.pretty_print();
                 println!("{}", *m);
                 println!("{:?}", b.next_legal);
                 println!("{:?}", moves);
                 println!("{:?}", b.get_moves());
                 assert!(false);
             }
             if depth > 1 {
                get_moves_at_depths_undo(b, depth - 1, out);
             }
             assert!(b.undo_move());
             if depth == out_len  {
                //println!("{}: {}", *m, out[out_len - 1] - temp);
             }
             if moves != b.get_moves() {
                 println!("After undo: {:?}", b.get_moves());
                 println!("Before undo: {:?}", moves);
                 assert!(false);
             }
         }
     }

     /*fn get_moves_at_depths_thread(other_b: &Board, depth: usize, out: &mut Vec<usize>) {
         if depth == 0 {
             return;
         }
         let b = other_b.clone();
         let moves = b.get_moves();
         let out_len = out.len();
         out[out_len - depth] += moves.len();
         let threads = Vec::new();
         for m in &moves {
             assert!(b.make_move(*m));
             get_moves_at_depths_undo(b, depth - 1, out);
             assert!(b.undo_move());
             if moves != b.get_moves() {
                 println!("After undo: {:?}", b.get_moves());
                 println!("Before undo: {:?}", moves);
                 assert!(false);
             }
         }sdfasdf
     }*/

     #[test]
     #[ignore]
     fn test_move_gen_2lv() {
         let mut b = Board::new(2);
         let depth = 8; // actually depth + 1
         println!("Level\tMoves");
         let mut levels = Vec::new();
         for _i in 0..depth {
             levels.push(0);
         }
         let mut now = Instant::now();
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14];
        for m in moves {
            //b.make_move(m);
        }
         get_moves_at_depths_undo(&mut b, depth, &mut levels);
         println!("Search took {} seconds", now.elapsed().as_secs());
         for i in 0..depth {
             print!("{}", i);
             print!("\t");
             println!("{}", levels[i]);
         }
         b = Board::new(2);
         now = Instant::now();
         let lowest_depth_moves = get_moves_at_depths_no_vector(&mut b, depth);
         println!("Search took {} seconds", now.elapsed().as_secs());
         println!("Level {}: {}", depth - 1, lowest_depth_moves);
     }
}