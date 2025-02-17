#![allow(unused)]

use printer::Printer;

mod printer;
mod chars;
mod writer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Použití: {} <text>", args[0]);
        std::process::exit(1);
    }

    let input = args[1].clone();
    let mut printer = Printer::init(input).unwrap();
    printer.start_drawing();
}
