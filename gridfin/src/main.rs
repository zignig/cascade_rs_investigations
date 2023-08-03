use clap::Parser;
use opencascade::primitives::Shape;

mod gridfin;

use crate::gridfin::{full, BasePlate};

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
    #[arg(short)]
    base: bool,
}
fn main() {
    let cli = Cli::parse();
    println!("generate");
    println!("{:#?}", cli);
    let prefix: String;
    let f: Shape;
    if cli.base {
        let mut bp = BasePlate::new(cli.width, cli.length);
        f = bp.shape();
        prefix = "base".to_owned();
    } else {
        f = full(cli.width, cli.length, cli.height);
        prefix = "gf".to_owned();
    }
    let ext: String;
    if cli.step {
        ext = "step".to_owned();
    } else {
        ext = "stl".to_owned();
    }
    let name = format!("{}_{}x{}x{}.{}",prefix, cli.width, cli.length, cli.height, ext);
    println!("output : {:?}", name);
    if cli.step {
        f.write_step(name).unwrap();
    } else {
        f.write_stl(name).unwrap();
    }
}
