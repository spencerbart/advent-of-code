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
        (2, 1) => d02::p01(),
        (2, 2) => d02::p02(),
        (3, 1) => d03::p01(),
        (3, 2) => d03::p02(),
        (4, 1) => d04::p01(),
        (4, 2) => d04::p02(),
        (5, 1) => d05::p01(),
        (5, 2) => d05::p02(),
        (6, 1) => d06::p01(),
        (6, 2) => d06::p02(),
        (7, 1) => d07::p01(),
        (7, 2) => d07::p02(),
        (8, 1) => d08::p01(),
        (8, 2) => d08::p02(),
        (9, 1) => d09::p01(),
        (9, 2) => d09::p02(),
        (10, 1) => d10::p01(),
        (10, 2) => d10::p02(),
        (11, 1) => d11::p01(),
        (11, 2) => d11::p02(),
        (12, 1) => d12::p01(),
        (12, 2) => d12::p02(),
        (13, 1) => d13::p01(),
        (13, 2) => d13::p02(),
        (14, 1) => d14::p01(),
        (14, 2) => d14::p02(),
        (15, 1) => d15::p01(),
        (15, 2) => d15::p02(),
        (16, 1) => d16::p01(),
        (16, 2) => d16::p02(),
        (17, 1) => d17::p01(),
        (17, 2) => d17::p02(),
        (18, 1) => d18::p01(),
        (18, 2) => d18::p02(),
        (19, 1) => d19::p01(),
        (19, 2) => d19::p02(),
        (20, 1) => d20::p01(),
        (20, 2) => d20::p02(),
        (21, 1) => d21::p01(),
        (21, 2) => d21::p02(),
        (22, 1) => d22::p01(),
        (22, 2) => d22::p02(),
        (23, 1) => d23::p01(),
        (23, 2) => d23::p02(),
        (24, 1) => d24::p01(),
        (24, 2) => d24::p02(),
        (25, 1) => d25::p01(),
        (25, 2) => d25::p02(),
        _ => println!("Day {} part {} not implemented", args.day, args.part),
    }
}
