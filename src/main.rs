#![allow(unused)]

use printer::Printer;

mod printer;
mod chars;
mod writer;

fn main() {
    let mut printer = Printer::init().unwrap();

    printer.start_drawing();
}
