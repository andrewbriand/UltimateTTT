use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Player {
    X,
    O,
    // Neither player has played in this square yet
    NEITHER,
    // Neither player has played in this square,
    // but it can never be played in because it is
    // in a higher occupied square
    DEAD,
}


#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Square {
    pub top_left: usize,
    pub level: usize,
}
// Each tile of the tic tac toe board is assigned an integer
// One level:
// 0 1 2
// 3 4 5
// 6 7 8
// Two levels:
// 00 01 02 03 04 05 06 07 08
// 09 10 11 12 13 14 15 16 17
// 18 19 20 21 22 23 24 25 26
// 27 28 29 30 31 32 33 34 35
// 36 37 38 39 40 41 42 43 44
// 45 46 47 48 49 50 51 52 53
// 54 55 56 57 58 59 60 61 62
// 63 64 65 66 67 68 69 70 71
// 72 73 74 75 76 77 78 79 80
// In the above example, (space: 0, level 1) is the square with its
// top left corner at 00 and its bottom right corner at 20
pub struct Board {
    // the width and height of the board
    size: usize,
    // the number of level in the board where
    // 1 is a standard 3x3 tic-tac-toe board
    levels: usize,
    // The player who will make the next move
    to_move: Player,
    // The squares that are occupied
    // Where the first element in the pair is the top left space of that square
    // the second element is the level
    occupied: HashMap<Square, Player>,
    // Tuple describing the upper left corner and level
    // of the next legal move space
    next_legal: Square,
    // The player that has won the game, or NEITHER if
    // the game is still ongoing
    pub winner: Player,
}

impl Board {
    // Creates a new board with size size_
    // size_ must be a power of three greater than or equal to 3
    pub fn new(levels_: usize) -> Board {
        if levels_ < 1 {
            panic!("levels_ must be >= 1");
        }
        let size_ = (3 as usize).pow(levels_ as u32);
        let mut result = Board {
                to_move: Player::X,
                size:  size_,
                occupied: HashMap::new(),
                next_legal: Square { top_left: 0, level: levels_},
                levels: levels_,
                winner: Player::NEITHER };
        for i in 0..(size_*size_) {
            result.occupied.insert(Square {top_left: i, level: 0}, 
                                   Player::NEITHER);
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
        for y in [0, 3, 6, 27, 30, 33, 54, 57, 60].iter() {
            for x in [0, 1, 2, 9, 10, 11, 18, 19, 20].iter() {
                let i = *y + *x;
                match self.occupied.get(&Square {top_left: i, level: 0}) {
                    Some(Player::X) => print!("X "),
                    Some(Player::O) => print!("O "),
                    Some(Player::NEITHER) => print!("- "),
                    Some(Player::DEAD) => print!("+ "),
                    None => panic!("Invalid board state"),
                }
            }
            println!("");
        }
    }

    fn bottom_right(&self, sqr: Square) -> usize {
        if sqr.level == 0 {
            sqr.top_left
        } else {
            self.bottom_right(self.descend(&sqr, 8))
        }
    }

    // Is the given space in the move bounds for this turn?
    fn in_bounds(&self, space: usize) -> bool {
       //println!("{}", space);
       //println!("{}", self.space_lvl_to_top_left(space, self.next_legal.1));
       space >= self.next_legal.top_left && 
       space <= self.bottom_right(self.next_legal)
    }

    // make the next move on space space
    // returns true iff the move is legal
    // does not affect board state if the move is illegal
    pub fn make_move(&mut self, space: usize) -> bool {
        let curr_sqr = Square {top_left: space, level: 0};
        // Make sure this square is available
        if *self.occupied.get(&curr_sqr).unwrap() != Player::NEITHER {
            return false;
        }
        // Check if the move is in the legal bounds
        if !self.in_bounds(space) {
            return false;
        }
        // Write this move to the board
        self.occupied.insert(curr_sqr, self.to_move);
        
        // Update occupied
        let (mut _check_sqr, _) = self.ascend(&curr_sqr);
        let &mut check_sqr = &mut _check_sqr;
        // Keep checking levels as long as the player made a capture
        while check_sqr.level <= self.levels {
            //let top_left = self.space_lvl_to_top_left(space, curr_level);
            let victorious_player = self.check_victory(&check_sqr);
            if victorious_player != Player::NEITHER {
                // This player now occupies this square
                self.occupied.insert(check_sqr, victorious_player);
                let mut downward_movement = 0;
                // Loop through all the level 0 spaces this square
                // occupies and write Player::DEAD to them if they 
                // are currently open
                /*while downward_movement < self.size * (3 as usize).pow(curr_level as u32) {
                    for i in 0..(3 as usize).pow(curr_level as u32) {
                        if *self.occupied.get(&(top_left + downward_movement + i, 0)).unwrap() == Player::NEITHER {
                            self.occupied.insert((top_left + downward_movement + i, 0), Player::DEAD);
                        }
                    }
                    downward_movement += self.size;
                }*/
                // If this is the top level, the capturing player
                // wins the game
                if check_sqr.level == self.levels {
                    self.winner = victorious_player;
                }
            } else {
                // No capture was made at this level, so stop checking
                // and update the bounds for the next move accordingly
                // No captures were made
                if check_sqr.level == 1 {
                    let (_, i) = self.ascend(&curr_sqr);
                    let (highest_sqr, _) = self.ascend(&check_sqr);
                    self.next_legal = self.descend(&highest_sqr, i);
                }
               /* if check_sqr.level == self.levels - 1 {
                    self.next_legal = self.ascend();
                } else {
                }*/
                break;
            }
            let (check_sqr, _) = self.ascend(&check_sqr);
        }

        // Move to the next player
        if self.to_move == Player::X {
            self.to_move = Player::O;
        } else {
            self.to_move = Player::X;
        }
        return true;
    }

   // Transforms with respect to a top-left square at a given level
   // where i is one of
   // 0 1 2
   // 3 4 5
   // 6 7 8
   // For example in the two-level board:
   // 00 01 02  03 04 05  06 07 08
   // 09 10 11  12 13 14  15 16 17
   // 18 19 20  21 22 23  24 25 26
   //
   // 27 28 29  30 31 32  33 34 35
   // 36 37 38  39 40 41  42 43 44
   // 45 46 47  48 49 50  51 52 53
   //
   // 54 55 56  57 58 59  60 61 62
   // 63 64 65  66 67 68  69 70 71
   // 72 73 74  75 76 77  78 79 80
   // space_from_lvl(0, 0, 1) would return 01
   // while space_from_lvl(0, 1, 1) would return 03
   fn descend(&self, sqr: &Square, i: usize) -> Square {
        Square { top_left: sqr.top_left + 
                      i * (3 as usize).pow(2 * (sqr.level - 1) as u32), 
                level: sqr.level - 1}
   } 

   fn ascend(&self, sqr: &Square) -> (Square, usize) {
       let f = sqr.top_left % (3 as usize).pow(2 * (sqr.level + 1) as u32);
       let i = f / (3 as usize).pow((sqr.level*2) as u32);
       let tl = sqr.top_left - i * (3 as usize).pow(2 * (sqr.level) as u32);
       (Square {top_left: tl, level: sqr.level + 1}, i)
   }

    // Determine if the square with space at its top left corner at level
    // where 0 is the lowest level (i.e. individual squares) has been 
    // won by a player
    // Returns the winner if so, returns NEITHER otherwise
    pub fn check_victory(&self, sqr: &Square) -> Player {
        let mut this_board: Vec<Player> = Vec::with_capacity(9); 
        let mut counter = 0;
        this_board.resize_with(9, || {
            let i = counter;
            counter += 1;
            let curr_sqr = self.descend(sqr, i);
            match self.occupied.get(&(curr_sqr)) {
                Some(p) => *p,
                None => Player::NEITHER
            }
        });
        
        // Check the horizontals
        for j in [0, 3, 6].iter() {
            let r = *j;
            if this_board[r] == this_board[r + 1] &&
               this_board[r + 1] == this_board[r + 2] {
                return this_board[r];
            }
        }

        // Check the verticals
        for j in [0, 1, 2].iter() {
            let r = *j;
            if this_board[r] == this_board[r + 3] &&
               this_board[r + 3] == this_board[r + 6] {
                return this_board[r];
            }
        }

        // Check the diagonals
        if this_board[0] == this_board[4] &&
           this_board[4] == this_board[8] {
               return this_board[0];
        }

        if this_board[2] == this_board[4] &&
           this_board[4] == this_board[6] {
               return this_board[2];
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
        assert!(b.make_move(0));
        assert!(!b.make_move(0));
        assert!(b.make_move(1));
     }

     #[test]
     fn test_basic_victory_2lv() {
        let mut b = Board::new(2);
        assert!(b.make_move(25));

        assert!(b.make_move(0));

        assert!(b.make_move(28));

        assert!(b.make_move(10));

        assert!(b.make_move(22));

        assert!(b.make_move(20));
        assert!(b.check_victory(0, 1) == Player::O);
     }

     #[test]
     fn test_basic_victory_2lv_2() {
        let mut b = Board::new(2);
        b.make_move(25);

        b.make_move(0);

        b.make_move(28);

        b.make_move(1);

        b.make_move(22);

        b.make_move(2);
        assert!(b.check_victory(0, 1) == Player::O);
     }

     #[test]
     fn test_basic_victory_2lv_3() {
        let mut b = Board::new(2);
        b.make_move(25);

        b.make_move(0);

        b.make_move(28);

        b.make_move(1);

        b.make_move(22);

        assert!(b.check_victory(0, 1) == Player::NEITHER);
     }
}