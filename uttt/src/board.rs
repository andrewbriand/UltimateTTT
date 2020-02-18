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
    occupied: HashMap<(usize, usize), Player>,
    // Tuple describing the upper left corner and level
    // of the next legal move space
    next_legal: (usize, usize),
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
                next_legal: (0, levels_ ),
                levels: levels_,
                winner: Player::NEITHER };
        for i in 0..(size_*size_) {
            result.occupied.insert((i, 0), Player::NEITHER);
        }
        return result;
    }

    pub fn pretty_print(&self) {
        for i in 0..(self.size*self.size) {
            match self.occupied.get(&(i, 0)) {
                Some(Player::X) => print!("X "),
                Some(Player::O) => print!("O "),
                Some(Player::NEITHER) => print!("- "),
                Some(Player::DEAD) => print!("+ "),
                None => panic!("Invalid board state"),
            }
            if i % self.size == self.size - 1 {
                println!("");
            }
        }
    }

    // Gets the top left corner of the square that space is
    // at level
    fn space_lvl_to_top_left(&self, space: usize, level: usize) -> usize {
       let _3pl = (3 as usize).pow(level as u32);
       let _3pln = self.size * _3pl;
       let i = space - (space % _3pl);
       let l = i - (i % self.size);
       (l - (l % _3pln)) + (i % self.size)
    }

    fn space_lvl_to_i(&self, space: usize, level: usize) -> usize {
       let _3pl = (3 as usize).pow(level as u32);
       let _3pln = self.size * _3pl;
       let i = space - (space % _3pl);
       if level != 0 {
           let l = i - (i % self.size);
           ((space % self.size) / _3pl) + 3 * (l / _3pln)
       } else {
           (i % 3) + 3 * (i/self.size)
       }
    }

    // Is the given space in the move bounds for this turn?
    fn in_bounds(&self, space: usize) -> bool {
       //println!("{}", space);
       //println!("{}", self.space_lvl_to_top_left(space, self.next_legal.1));
       self.space_lvl_to_top_left(space, self.next_legal.1)  == self.next_legal.0
    }

    // make the next move on space space
    // returns true iff the move is legal
    // does not affect board state if the move is illegal
    pub fn make_move(&mut self, space: usize) -> bool {
        // Make sure this square is available
        if *self.occupied.get(&(space, 0)).unwrap() != Player::NEITHER {
            return false;
        }
        // Check if the move is in the legal bounds
        if !self.in_bounds(space) {
            return false;
        }
        // Write this move to the board
        self.occupied.insert((space, 0), self.to_move);
        
        // TODO: Update occupied and next_legal
        // Update occupied
        let mut curr_level = 1;
        // Keep checking levels as long as the player made a capture
        while curr_level <= self.levels {
            let top_left = self.space_lvl_to_top_left(space, curr_level);
            let victorious_player = self.check_victory(top_left, curr_level);
            if victorious_player != Player::NEITHER {
                // This player now occupies this square
                self.occupied.insert((top_left, curr_level), victorious_player);
                let mut downward_movement = 0;
                // Loop through all the level 0 spaces this square
                // occupies and write Player::DEAD to them if they 
                // are currently open
                while downward_movement < self.size * (3 as usize).pow(curr_level as u32) {
                    for i in 0..(3 as usize).pow(curr_level as u32) {
                        if *self.occupied.get(&(top_left + downward_movement + i, 0)).unwrap() == Player::NEITHER {
                            self.occupied.insert((top_left + downward_movement + i, 0), Player::DEAD);
                        }
                    }
                    downward_movement += self.size;
                }
                // If this is the top level, the capturing player
                // wins the game
                if curr_level == self.levels {
                    self.winner = victorious_player;
                }
            } else {
                // No capture was made at this level, so stop checking
                // and update the bounds for the next move accordingly
                if curr_level == self.levels - 1 {
                    println!("curr_level: {}", curr_level);
                    let i = self.space_lvl_to_i(space, curr_level - 1);
                    println!("i: {}", i);
                    let new_top_left = self.space_lvl_to_top_left(top_left, curr_level + 1);
                    self.next_legal = (self.space_from_lvl(new_top_left, curr_level, i), curr_level);
                    println!("next_legal: {:?}", self.next_legal);
                } else {
                    let i = self.space_lvl_to_i(top_left, curr_level);
                    self.next_legal = (self.space_from_lvl(top_left, curr_level + 1, i), curr_level + 1);
                }
                break;
            }
            curr_level += 1;
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
   fn space_from_lvl(&self, top_left: usize, level: usize, i: usize) -> usize {
       let x_movement = (i % 3) * (3 as usize).pow(level as u32);
       let y_movement = self.size * (i/3) * (3 as usize).pow(level as u32);
       return top_left + x_movement + y_movement;
   } 

    // Determine if the square with space at its top left corner at level
    // where 0 is the lowest level (i.e. individual squares) has been 
    // won by a player
    // Returns the winner if so, returns NEITHER otherwise
    pub fn check_victory(&self, top_left: usize, level: usize) -> Player {
        let mut this_board: Vec<Player> = Vec::with_capacity(9); 
        let mut counter = 0;
        this_board.resize_with(9, || {
            let i = counter;
            counter += 1;
            match self.occupied.get(&(self.space_from_lvl(top_left, level - 1, i), level - 1)) {
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

    // Determine the next legal move set
    fn update_next_legal(&mut self) {

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