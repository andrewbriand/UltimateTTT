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
                next_legal: (0, levels_ - 1),
                levels: levels_};
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

    // make the next move on space space
    // returns true iff the move is legal
    // does not affect board state if the move is illegal
    pub fn make_move(&mut self, space: usize) -> bool {
        // Make sure this square is available
        if *self.occupied.get(&(space, 0)).unwrap() != Player::NEITHER {
            return false;
        }
        let size = self.size;
        // Check if the move is in the legal bounds
        /*if self.next_legal.0 / size > space / size ||
           self.next_legal.0 % size > space % size ||
           self.next_legal.1 / size < space / size ||
           self.next_legal.1 % size < space % size {
            return false;
        }*/
        // Write this move to the board
        self.occupied.insert((space, 0), self.to_move);
        
        // TODO: Update occupied and next_legal


        // Move to the next player
        if self.to_move == Player::X {
            self.to_move = Player::O;
        } else {
            self.to_move = Player::X;
        }
        return true;
    }

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
        println!("{:?}", this_board);
        

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