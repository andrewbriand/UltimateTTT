pub use crate::board::Board;
pub use crate::ai::AI;
use text_io::read;

pub struct HumanPlayer {
    board: Board,
}

impl AI for HumanPlayer {
    fn get_move(&mut self, last_move : i64) -> i64 {
        if last_move != -1 {
            self.board.make_move(last_move as usize);
        }
        self.board.pretty_print();
        loop {
            println!("{:?} to move", self.board.get_to_move());
            println!("Enter a square or 900 to resign: ");
            let i: usize = read!();
            if i == 900 {
                return -1;
            } else {
                if self.board.make_move(i as usize) {
                    return i as i64;
                } else {
                    println!("{} is an illegal move", i);
                }
            }
        }
    }

    fn cleanup(&mut self) {}
}

impl HumanPlayer {
    pub fn new(max_level_: usize) -> HumanPlayer {
        HumanPlayer { board: Board::new(max_level_) }
    }
}