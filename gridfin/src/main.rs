
use clap::Parser;

mod gridfin;

use crate::gridfin::full;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    length: usize,
    #[arg(short, long, default_value_t = 1)]
    width: usize,
    #[arg(long, default_value_t = 1)]
    height: usize,
    #[arg(short)]
    step: bool,
}
fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    let f = full(cli.width, cli.length, cli.height);
    let ext: String;
    if cli.step {
        ext = "step".to_owned();
    } else {
        ext = "stl".to_owned();
    }
    let name = format!("gf_{}x{}x{}.{}", cli.width, cli.length, cli.height, ext);
    println!("{:?}", name);
    if cli.step {
        f.write_step(name).unwrap();
    } else {
        f.write_stl(name).unwrap();
    }
}
