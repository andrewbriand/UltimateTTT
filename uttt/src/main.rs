use structopt::StructOpt;

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
}
