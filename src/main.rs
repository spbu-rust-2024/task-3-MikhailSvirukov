mod options;
mod side_funcs;
mod math;
mod variants;

use crate::variants::var::INPUT_OPTIONS;
use crate::variants::var::OPTION_COUNT;
use ansi_term::Color::{Green, Red};
use ansi_term::Style;
use std::io;

macro_rules! exit {
    () => {
        INPUT_OPTIONS[16];
    };
}
fn main() {
    println!(
        "{} {}",
        Style::new().bold().paint("Choose input variant:"),
        Style::new().italic().paint("\nfile\nprint")
    );
    let mut input_option: String = String::new();
    let mut array: Vec<f64> = Vec::new();
    io::stdin()
        .read_line(&mut input_option)
        .expect("Failed to read line");
    match input_option.as_str() {
        "print\n" => {
            println!("{}{}", term_cursor::Up(1), Green.bold().paint("print"));
            array = side_funcs::input_array()
        }
        "file\n" => {
            array = {
                println!("{}{}", term_cursor::Up(1), Green.bold().paint("file"));
                side_funcs::input_array_file()
            }
        }
        _ => {
            println!("{}", Red.paint("No such option"));
            return;
        }
    };
    side_funcs::sorting(&mut array);
    let mut status = INPUT_OPTIONS[OPTION_COUNT-1];

    loop {
        status = INPUT_OPTIONS[OPTION_COUNT-1];
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        for item in INPUT_OPTIONS {
            if line.trim() == item {
                status = item;
            }
        }
        if status == exit!() {
            println!("{}{}", term_cursor::Up(1), Red.bold().paint("exit"));
            break;
        } else {
            options::scen(status, &mut array);
        }

        while array.is_empty() {
            println!("{}", Red.underline().paint("Empty array, input new one:"));
            array = side_funcs::input_array();
        }
    }
}
