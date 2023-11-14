use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Day to run
    #[clap(short, long)]
    pub day: u8,

    // Part to run
    #[clap(short, long, default_value = "1")]
    pub part: u8,
}

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

pub fn run() {
    let args = Args::parse();
    match (args.day, args.part) {
        (1, 1) => d01::p01(),
        (1, 2) => d01::p02(),
        _ => println!("Day {} part {} not implemented", args.day, args.part),
    }
}
