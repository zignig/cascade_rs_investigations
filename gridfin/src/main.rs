use clap::Parser;
use opencascade::primitives::Shape;

mod gridfin;

use crate::gridfin::{full, BasePlate, Plate,Connector};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// length of the unit
    #[arg(short, long, default_value_t = 1)]
    length: usize,
    /// width of the unit
    #[arg(short, long, default_value_t = 1)]
    width: usize,
    /// height of the unit
    #[arg(short, long, default_value_t = 1)]
    depth: usize,
    /// export a .step file or an .stl
    #[arg(short, long)]
    step: bool,
    /// make a base plate
    #[arg(short, long)]
    base: bool,
}
fn main() {
    let cli = Cli::parse();
    println!("generate");
    println!("{:#?}", cli);
    let prefix: String;
    let f: Shape;
    // Is it a base plate ?
    if cli.base {
        let mut bp = BasePlate::new(cli.width, cli.length);
        f = bp.shape();
        prefix = "base".to_owned();
    // make an basic module
    } else {
        //let mut pl = Connector::new(cli.width,cli.length,Connector::UNDER);
        //f = pl.shape();
        f = full(cli.width, cli.length, cli.depth);
        prefix = "gf".to_owned();
    }
    // save the generated build
    let ext: String;
    if cli.step {
        ext = "step".to_owned();
    } else {
        ext = "stl".to_owned();
    }
    // make a working filename
    let name = format!(
        "{}_{}x{}x{}.{}",
        prefix, cli.width, cli.length, cli.depth, ext
    );
    // Build the file
    // this needs STL resolution
    println!("output : {:?}", name);
    if cli.step {
        f.write_step(name).unwrap();
    } else {
        f.write_stl_with_tolerance(name, 0.1).unwrap();
        //f.write_stl(name).unwrap();
    }
}
