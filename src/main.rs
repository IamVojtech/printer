#![allow(unused)]

use printer::Printer;

mod printer;
mod chars;
mod writer;

fn main() {
    let mut input = String::new();
    print!("Zadej text k vytištění: ");
    let b1 = std::io::stdin().read_line(&mut input).unwrap();

    let mut printer = Printer::init(input).unwrap();

    printer.start_drawing();
}
