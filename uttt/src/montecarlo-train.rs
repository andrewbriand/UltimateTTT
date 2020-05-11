mod ai;
//use ai::AI;

mod montecarloai;
use montecarloai::MonteCarloAI;

mod board;
//use board::Board;

fn main() {
    let mut mcai = MonteCarloAI::from_save("test1.mcai".to_string());
    mcai.train(10);
    mcai.save_to_file("test1.mcai".to_string());
}