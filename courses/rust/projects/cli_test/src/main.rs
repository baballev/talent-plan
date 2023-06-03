// Small implementation of UNIX's cat to practice CLI programs in rust
extern crate clap;
use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.path).unwrap();
    for line in file.lines() {
        println!("{line}");
    }
}
