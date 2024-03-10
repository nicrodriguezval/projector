use clap::Parser;
use projector::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
