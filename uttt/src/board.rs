use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
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
    // The smallest spaces on the board and who occupies them
    // counted in row-major order
    spaces: Vec<Player>,
    size: usize,
    // The player who will make the next move
    to_move: Player,
    // The higher level squares that are occupied
    // Where the first element in the pair is the top left space
    // the second element is the level
    occupied: HashMap<(usize, usize), Player>,
    // Tuple describing the upper left corner and level
    // of the next legal move space
    next_legal: (usize, usize),
}

impl Board {
    // Creates a new board with size size_
    // size_ must be a power of three greater than or equal to 3
    pub fn new(size_: usize) -> Board {
        let mut result = Board { spaces: Vec::new(),
                to_move: Player::X,
                size:  size_,
                occupied: HashMap::new(),
                next_legal: (0, size_*size_ - 1)};
        result.spaces.resize_with(size_*size_, || {Player::NEITHER});
        return result;
    }

    pub fn pretty_print(&self) {
        println!("hello");
        for (i, space) in self.spaces.iter().enumerate() {
            match space {
                Player::X => print!("X "),
                Player::O => print!("O "),
                Player::NEITHER => print!("- "),
                Player::DEAD => print!("+ "),
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
        if self.spaces[space] != Player::NEITHER {
            return false;
        }
        let size = self.size;
        // Check if the move is in the legal bounds
        if self.next_legal.0 / size > space / size ||
           self.next_legal.0 % size > space % size ||
           self.next_legal.1 / size < space / size ||
           self.next_legal.1 % size < space % size {
            return false;
        }
        // Write this move to the board
        self.spaces[space] = self.to_move;
        
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
    // where 0 is the lowest level (i.e. squares of 9 space) has been 
    // won by a player
    // Returns the winner if so, returns NEITHER otherwise
    pub fn check_victory(&self, top_left: usize, level: usize) -> Player {
        let mut this_board: Vec<Player> = Vec::with_capacity(9); 
        let mut counter = 0;
        this_board.resize_with(9, || {
            let i = counter;
            counter += 1;
            if level == 0 {
                self.spaces[self.space_from_lvl(top_left, level, i)]
            } else {
                match self.occupied.get(&(top_left, level)) {
                    Some(p) => *p,
                    None => Player::NEITHER
                }
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
        let mut b = Board::new(9);
        assert!(b.make_move(0));
        assert!(!b.make_move(0));
        assert!(b.make_move(1));
     }
}