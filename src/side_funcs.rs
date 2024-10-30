use crate::variants::var::{INPUT_OPTIONS};
use ansi_term::Style;
use std::{fs, io};
pub fn input_array() -> Vec<f64> {
    println!("{}", Style::new().bold().paint("Input numbers:"));
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let array: Vec<f64> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().expect("not a number"))
        .collect();
    array
}

pub fn input_array_usize() -> Vec<usize> {
    println!("{}", Style::new().bold().paint("Input numbers:"));
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let array: Vec<usize> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().expect("not a number"))
        .collect();
    array
}
pub fn input_array_file() -> Vec<f64> {
    println!("{}", Style::new().bold().paint("Input path to file:"));
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let line = fs::read_to_string(line.trim()).expect("Should have been able to read the file");
    let array: Vec<f64> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().expect("not a number"))
        .collect();
    array
}

pub fn sorting(array: &mut [f64]) {
    let mut counter = 1;
    while counter < array.len() {
        let mut i = counter;
        while i > 0 {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
            }
            i -= 1;
        }
        counter += 1;
    }
}

pub fn print_man() {
    println!("{}", Style::new().bold().paint("This is a basic statistics application for calculation of some functions.\nThis is the list of options available:"));
    for item in INPUT_OPTIONS {
        println!("--{}", Style::new().italic().paint(item));
    }
}
