use crate::variants::INPUT_OPTIONS;
use std::{fs, io};
use ansi_term::Color::{Green, Red};
use ansi_term::Style;
use crate::lib::formuls;
macro_rules! arithmetic {() => {0}}
macro_rules! generalized {() => {1}}
macro_rules! geometric {() => {2}}
macro_rules! arithmeticgeometric {() => {3}}
macro_rules! arithmeticgeometricmodified {() => {4}}
macro_rules! trancated {() => {5}}
macro_rules! winsorized {() => {6}}
macro_rules! median {() => {7}}
macro_rules! mode {() => {8}}
macro_rules! absolutedeviation {() => {9}}
macro_rules! standarddeviation {() => {10}}
macro_rules! linearcoefficient {() => {11}}
macro_rules! standardcoefficient {() => {12}}
macro_rules! variance {() => {13}}
macro_rules! help {() => {14}}
macro_rules! exit {() => {15}}
macro_rules! add {() => {16}}
macro_rules! addfile {() => {17}}
macro_rules! excludeindex {() => {18}}

macro_rules! excludevalue {() => {19}}
macro_rules! excludemax {() => {20}}
macro_rules! excludemin {() => {21}}
macro_rules! new {() => {22}}
macro_rules! printdata {() => {23}}


pub fn scen(status: usize, array: &Vec<f64>) {
    match status {
        arithmetic!() => {
            println!("{} {}", formuls::mean_arithmetic(array), term_cursor::Up(2));
            println!("{} {}",Green.bold().paint("mean arithmetic"), term_cursor::Down(2));
        },
        generalized!() => {
            println!("{}", Style::new().bold().paint("Input exponent:"));
            let mut line=String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb=line.trim().parse::<f64>().expect("no digit");
            println!("{} {}", formuls::mean_generalized(array, numb), term_cursor::Up(2));
            println!("{} {}",Green.bold().paint("mean generalized"), term_cursor::Down(2));
        },
        geometric!() => {
            println!("{} {}", formuls::mean_geometric(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("mean geometric"), term_cursor::Down(2));
        }
        arithmeticgeometric!() => {
            if array.len() != 2 {println!("{}", Red.paint("Expect two values"))}
            else {
                println!("{}", Style::new().bold().paint("Input number of steps expected:"));
                let mut line=String::new();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let numb=line.trim().parse::<usize>().expect("no digit");
                println!("{} {}", formuls::mean_arithmetic_geometric(array, numb), term_cursor::Up(2));
                println!("{} {}", Green.bold().paint("mean arithmetic geometric"), term_cursor::Down(2));
            }
        }
        arithmeticgeometricmodified!() => {
            if array.len() != 2 {println!("{}", Red.paint("Expect two values")) }
            else { println!("{} {}", formuls::mean_arithmetic_geometric_modified(array), term_cursor::Up(2));
                println!("{} {}", Green.bold().paint("mean arithmetic geometric modified"), term_cursor::Down(2));}
        }

        trancated!() => {
            println!("{}", Style::new().bold().paint("Input number of mins/maxes to exclude:"));
            let mut line=String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb=line.trim().parse::<usize>().expect("no digit");
            if (numb*2)>array.len() {println!("{}", Red.paint("Can not extract so many numbers"));}
            else {println!("{} {}", formuls::mean_truncated(array, numb), term_cursor::Up(2));
                println!("{} {}", Green.bold().paint("mean truncated"), term_cursor::Down(2));
                }
        }
        winsorized!() => {
            println!("{}", Style::new().bold().paint("Input number of mins/maxes to replace:"));
            let mut line=String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb=line.trim().parse::<usize>().expect("no digit");
            if numb*2>array.len() {println!("{}", Red.paint("Can not extract so many numbers"));}
            else {println!("{} {}", formuls::mean_winsorized(array, numb), term_cursor::Up(2));
                println!("{} {}", Green.bold().paint("mean winsorized"), term_cursor::Down(2));}
        }

        median!() => {
            println!("{} {}", formuls::median(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("median"), term_cursor::Down(2));},
        mode!() => {
            println!("{} {}", formuls::mode(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("mode"), term_cursor::Down(2));},
        absolutedeviation!() => {
            println!("{} {}", formuls::average_absolute_deviation(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("average absolute deviation"), term_cursor::Down(2));},
        standarddeviation!() => {
            println!("{} {}", formuls::standard_deviation(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("standard deviation"), term_cursor::Down(2));},
        linearcoefficient!() => {
            println!("{} {}", formuls::linear_variation_coefficient(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("linear variation coefficient"), term_cursor::Down(2));},
        standardcoefficient!() => {
            println!("{} {}", formuls::standard_variation_coefficient(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("standard variation coefficient"), term_cursor::Down(2));},
        variance!() => {
            println!("{} {}", formuls::variance(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("variance"), term_cursor::Down(2));},
        help!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("help"));
            print_man();
        }
        printdata!() => {

            for i in 0..array.len() {
                print!("{} ", array[i])
            }

            println!("\n{}{}{}",  term_cursor::Up(2) , Green.bold().paint("print_data"), term_cursor::Down(1));
        }


        _ => println!("{}", Red.paint("No valid argument\nTry 'help'"))
    }
}

pub fn scen_new_array(status: usize,array: &Vec<f64>)  -> Vec<f64>{
    let mut array_to_realloc: Vec<f64>=vec![];
    match status {
        add!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("add"));
            let new_array = input_array();
            for i in 0..array.len() {
                array_to_realloc.push(array[i]);
            }
            for i in 0..new_array.len() {
                array_to_realloc.push(new_array[i]);
            }
            sorting(&mut array_to_realloc);

        }

        addfile!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("add_from_file"));
            let new_array = input_array_file();
            for i in 0..array.len() {
                array_to_realloc.push(array[i]);
            }
            for i in 0..new_array.len() {
                array_to_realloc.push(new_array[i]);
            }
            sorting(&mut array_to_realloc);

        }

        excludevalue!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("exclude_number_by_value"));
            let numbers_exclude = input_array();
            let mut check=0;
            for i in 0..array.len() {
                check=0;
                for u in 0..numbers_exclude.len() {
                    if numbers_exclude[u]==array[i] {check=1; break;}
                }
                if check==0 {array_to_realloc.push(array[i]);}
            }
        }

        excludeindex!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("exclude_number_by_index"));
            let numbers_exclude = input_array();
            let mut check=0;
            for i in 0..array.len() {
                check=0;
                for u in 0..numbers_exclude.len() {
                    if numbers_exclude[u]==i as f64 {check=1; break;}
                }
                if check==0 {array_to_realloc.push(array[i]);}
            }

        }

        excludemax!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("exclude_maxes"));
            for i in 0..array.len() {
                if array[i]!=array[array.len()-1] {array_to_realloc.push(array[i]);}
            }
        }

        excludemin!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("exclude_mins"));
            for i in 0..array.len() {
                if array[i]!=array[0] {array_to_realloc.push(array[i]);}
            }
        }

        new!() => {
            println!("{}{}", term_cursor::Up(1) , Green.bold().paint("new_data"));
            array_to_realloc=input_array();
        }



        _ => println!("{}", Red.paint("No valid argument"))


    }
    array_to_realloc
}

pub fn input_array() -> Vec<f64> {
    println!("{}", Style::new().bold().paint("Input numbers:"));
    let mut line=String::new();
    io::stdin()
        .read_line( &mut line)
        .expect("Failed to read line");
    let array: Vec<f64>= line.trim().split(' ').map(|x| x.parse::<f64>().expect("not a number")).collect();
    array
}
pub fn input_array_file() -> Vec<f64> {
    println!("{}", Style::new().bold().paint("Input path to file:"));
    let mut line=String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let line = fs::read_to_string(line.trim())
        .expect("Should have been able to read the file");
    let array: Vec<f64>= line.trim().split(' ').map(|x| x.parse::<f64>().expect("not a number")).collect();
    array
}

pub fn sorting(array: &mut Vec<f64>) {
    let mut counter =1;
    while counter<array.len() {
        let mut i = counter;
        while i >0 {
            if array[i] < array[i - 1] {
                let t = array[i];
                array[i] = array[i - 1];
                array[i - 1] = t;
            }
            i -= 1;
        }
        counter+=1;
    }
}

pub fn print_man() {
    println!("{}", Style::new().bold().paint("This is a basic statistics application for calculation of some functions.\nThis is the list of options available:"));
    for i in 0..24 {
        println!("--{}", Style::new().italic().paint(INPUT_OPTIONS[i]));
    }
}


