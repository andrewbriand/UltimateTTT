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
    let mut b = Board::new(2);
    b.pretty_print();
    println!("{:?}", args.x_ai_path);
    assert!(b.make_move(6));
    assert!(b.make_move(0));

    assert!(!b.make_move(28));
    assert!(b.make_move(9));
    assert!(b.make_move(10));

    assert!(!b.make_move(22));
    assert!(b.make_move(31));
    assert!(b.make_move(20));

    assert!(!b.make_move(37));
    assert!(b.make_move(79));
    assert!(b.make_move(30));

    assert!(!b.make_move(36));
    assert!(b.make_move(19));
    assert!(b.make_move(40));

    assert!(!b.make_move(5));
    assert!(b.make_move(39));
    assert!(b.make_move(50));

    assert!(!b.make_move(64));
    assert!(b.make_move(62));
    assert!(b.make_move(60));

    assert!(!b.make_move(72));
    assert!(b.make_move(11));
    assert!(b.make_move(70));

    assert!(!b.make_move(77));
    assert!(b.make_move(32));
    assert!(b.make_move(80));
    b.pretty_print();
    println!("{:?}", b.winner);
}
