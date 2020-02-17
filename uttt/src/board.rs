use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Player {
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
    // the second element is the bottom right
    occupied: HashMap<(usize, usize), Player>,
    // Tuple describing the upper left and lower right corner
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
                next_legal: (0, size_ - 1)};
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
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
     fn test_basic_moves() {
        let mut b = Board::new(9);
        assert!(b.make_move(0));
        assert!(!b.make_move(0));
        assert!(b.make_move(1));
     }
}