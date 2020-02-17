use structopt::StructOpt;
mod board;
pub use board::Board;

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
    println!("{:?}", args.x_ai_path);
    let mut b = Board::new(9);
    assert!(b.make_move(0));
    b.pretty_print();
    b.make_move(1);
    b.pretty_print();
}
