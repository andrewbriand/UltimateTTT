mod ai;
//use ai::AI;

mod montecarloai;
use montecarloai::MonteCarloAI;

mod board;
//use board::Board;

fn main() {
    //let mut mcai = MonteCarloAI::new();
    let mut mcai = MonteCarloAI::from_save("test1.mcai".to_string());
    //mcai.train(20000);
    //mcai.save_to_file("test1.mcai".to_string());
}