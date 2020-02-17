use std::collections::HashMap;

enum Player {
    X,
    O,
    NEITHER,
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
}

impl Board {
    pub fn new(size_: usize) -> Board {
        let mut result = Board { spaces: Vec::new(),
                to_move: Player::X,
                size:  size_,
                occupied: HashMap::new() };
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
            }
            if i % self.size == self.size - 1 {
                println!("");
            }
        }
    }
}
