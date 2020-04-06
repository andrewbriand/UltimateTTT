use structopt::StructOpt;
mod board;
pub use board::Board;
use board::Player;
use text_io::read;

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
    let mut b = Board::new(2);
    b.pretty_print();
    println!("{:?}", args.x_ai_path);
    let mut v = Vec::new();
    loop {
        let i: usize = read!();
        if i == 900 {
            break;
        }
        if i == 901 {
            println!("{}", b.undo_move());
        } else {
            println!("{}", b.make_move(i));
            v.push(i);
        }
        //println!("{:?}", b);
        b.pretty_print();
        if b.winner != Player::NEITHER {
            break;
        }
        println!("{:?}", b.next_legal);
    }
    println!("{:?} wins", b.winner);
    println!("{:?}", v);
}
