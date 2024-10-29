mod lib;
mod variants;
mod options;
use std::io;
use ansi_term::Color::{Green, Red};
use ansi_term::Style;

macro_rules! printdata {() => {23}}
macro_rules! exit {() => {15}}
fn main() {
    println!("{} {}", Style::new().bold().paint("Choose input variant:"), Style::new().italic().paint("\nfile\nprint"));
    let mut input_option:String=String::new();
    let mut array:Vec<f64>=Vec::new();
    io::stdin()
        .read_line(&mut input_option)
        .expect("Failed to read line");
    match input_option.as_str() {
        "print\n" => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("print"));
            array = options::input_array()
        },
        "file\n" => array= {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("file"));
            options::input_array_file()
        },
        _ => {println!("{}", Red.paint("No such option"));
            return;
        }
    };
    options::sorting(&mut array);
    let mut status = variants::OPTION_COUNT;

    loop {
        status=variants::OPTION_COUNT;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        for i in 0..variants::OPTION_COUNT {
            if line.trim() == variants::INPUT_OPTIONS[i] {
                status = i;
            }
        }
        if status==exit!() {
            println!("{}{}", term_cursor::Up(1) , Red.bold().paint("exit"));
            break;
        }
        else if status>exit!() && status<printdata!() {options::scen_new_array(status, &mut array);}
        else {options::scen(status, &array);}

        while array.is_empty() {
            println!("{}", Red.underline().paint("Empty array, input new one:"));
            array=options::input_array();
        }
    }
}




