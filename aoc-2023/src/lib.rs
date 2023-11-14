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
        (2, 1) => d02::p01(),
        (3, 1) => d03::p01(),
        (4, 1) => d04::p01(),
        (5, 1) => d05::p01(),
        (6, 1) => d06::p01(),
        (7, 1) => d07::p01(),
        (8, 1) => d08::p01(),
        (9, 1) => d09::p01(),
        (10, 1) => d10::p01(),
        (11, 1) => d11::p01(),
        (12, 1) => d12::p01(),
        (13, 1) => d13::p01(),
        (14, 1) => d14::p01(),
        (15, 1) => d15::p01(),
        (16, 1) => d16::p01(),
        (17, 1) => d17::p01(),
        (18, 1) => d18::p01(),
        (19, 1) => d19::p01(),
        (20, 1) => d20::p01(),
        (21, 1) => d21::p01(),
        (22, 1) => d22::p01(),
        (23, 1) => d23::p01(),
        (24, 1) => d24::p01(),
        (25, 1) => d25::p01(),
        _ => println!("Day {} part {} not implemented", args.day, args.part),
    }
}
