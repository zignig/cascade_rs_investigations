use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod gridfin;

use gridfin::full;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    length: usize,
    #[arg(short, long)]
    width: usize,
    #[arg(long)]
    height: usize,
    #[arg(short)]
    step: bool,
}
fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    //let mut mag = Magnet::new(dvec3(0.0, 0.0, 1.0));
    //let mut plate = Plate::new(4, 1).shape();
    //plate.write_stl("plate.stl").unwrap();
    //let lip = Base::lip(2,2,3);
    //lip.write_stl("lip.stl").unwrap();
    //let mut w = Wall::new(1, 1, 2, false);
    //w.shape().write_stl("wall.stl").unwrap();
    let f = full(cli.width, cli.length, cli.height);
    if cli.step {
        f.write_step("full.step").unwrap();
    } else {
        f.write_stl("full.stl").unwrap();
    }
}
