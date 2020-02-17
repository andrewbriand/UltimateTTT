use structopt::StructOpt;
mod board;
pub use board::Board;
use board::Player;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    o_ai_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    x_ai_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args(); 
    println!("{:?}", args.o_ai_path);
    let mut b = Board::new(9);
    b.pretty_print();
    println!("{:?}", args.x_ai_path);
    b.make_move(25);

    b.make_move(0);

    b.make_move(28);

    b.make_move(10);

    b.make_move(22);

    b.make_move(20);
    b.pretty_print();
}
