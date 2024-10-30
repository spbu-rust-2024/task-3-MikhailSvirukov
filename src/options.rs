use crate::lib::formuls;
use crate::side_funcs::{input_array, input_array_file, input_array_usize, print_man, sorting};
use ansi_term::Color::{Green, Red};
use ansi_term::Style;
use std::io;

macro_rules! arithmetic {
    () => {
        0
    };
}
macro_rules! generalized {
    () => {
        1
    };
}
macro_rules! geometric {
    () => {
        2
    };
}
macro_rules! arithmeticgeometric {
    () => {
        3
    };
}
macro_rules! arithmeticgeometricmodified {
    () => {
        4
    };
}
macro_rules! trancated {
    () => {
        5
    };
}
macro_rules! winsorized {
    () => {
        6
    };
}
macro_rules! median {
    () => {
        7
    };
}
macro_rules! mode {
    () => {
        8
    };
}
macro_rules! absolutedeviation {
    () => {
        9
    };
}
macro_rules! standarddeviation {
    () => {
        10
    };
}
macro_rules! linearcoefficient {
    () => {
        11
    };
}
macro_rules! standardcoefficient {
    () => {
        12
    };
}
macro_rules! variance {
    () => {
        13
    };
}
macro_rules! colmogorov {
    () => {
        14
    };
}
macro_rules! help {
    () => {
        15
    };
}
macro_rules! exit {
    () => {
        16
    };
}
macro_rules! add {
    () => {
        17
    };
}
macro_rules! addfile {
    () => {
        18
    };
}
macro_rules! excludevalue {
    () => {
        19
    };
}

macro_rules! excludeindex {
    () => {
        20
    };
}
macro_rules! excludemax {
    () => {
        21
    };
}
macro_rules! excludemin {
    () => {
        22
    };
}
macro_rules! new {
    () => {
        23
    };
}
macro_rules! printdata {
    () => {
        24
    };
}

pub fn scen(status: usize, array: &Vec<f64>) {
    match status {
        arithmetic!() => {
            println!("{} {}", formuls::mean_arithmetic(array), term_cursor::Up(2));
            println!(
                "{} {}",
                Green.bold().paint("mean arithmetic"),
                term_cursor::Down(2)
            );
        }
        generalized!() => {
            println!("{}", Style::new().bold().paint("Input exponent:"));
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb = line.trim().parse::<f64>().expect("no digit");
            println!(
                "{} {}",
                formuls::mean_generalized(array, numb),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("mean generalized"),
                term_cursor::Down(2)
            );
        }
        geometric!() => {
            println!("{} {}", formuls::mean_geometric(array), term_cursor::Up(2));
            println!(
                "{} {}",
                Green.bold().paint("mean geometric"),
                term_cursor::Down(2)
            );
        }
        arithmeticgeometric!() => {
            println!(
                "{}{}",term_cursor::Up(1),
                Green.bold().paint("mean arithmetic geometric"),
            );
            if array.len() != 2 {
                println!("{}", Red.paint("Expect two values"))
            } else {
                println!(
                    "{}",
                    Style::new().bold().paint("Input number of steps expected:")
                );
                let mut line = String::new();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let numb = line.trim().parse::<usize>().expect("no digit");
                println!(
                    "{}",
                    formuls::mean_arithmetic_geometric(array, numb)
                );

            }
        }
        arithmeticgeometricmodified!() => {
            println!(
                "{}{}",term_cursor::Up(1),
                Green.bold().paint("mean arithmetic geometric modified"),
            );
            if array.len() != 2 {
                println!("{}", Red.paint("Expect two values"))
            } else {
                println!(
                    "{}",
                    formuls::mean_arithmetic_geometric_modified(array),
                );

            }
        }

        trancated!() => {
            println!(
                "{} {}",
                Style::new()
                    .bold()
                    .paint("Input number of mins/maxes to exclude:"),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("mean truncated"),
                term_cursor::Down(2)
            );
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb = line.trim().parse::<usize>().expect("no digit");
            if (numb * 2) > array.len() {
                println!("{}", Red.paint("Can not extract so many numbers"));
            } else {
                println!(
                    "{}",
                    formuls::mean_truncated(array, numb),
                );
            }
        }
        winsorized!() => {
            println!(
                "{} {}",
                Style::new()
                    .bold()
                    .paint("Input number of mins/maxes to replace:"),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("mean winsorized"),
                term_cursor::Down(2)
            );
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let numb = line.trim().parse::<usize>().expect("no digit");
            if numb * 2 > array.len() {
                println!("{}", Red.paint("Can not extract so many numbers"));
            } else {
                println!(
                    "{}",
                    formuls::mean_winsorized(array, numb)
                );
            }
        }

        median!() => {
            println!("{} {}", formuls::median(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("median"), term_cursor::Down(2));
        }
        mode!() => {
            println!("{} {}", formuls::mode(array), term_cursor::Up(2));
            println!("{} {}", Green.bold().paint("mode"), term_cursor::Down(2));
        }
        absolutedeviation!() => {
            println!(
                "{} {}",
                formuls::average_absolute_deviation(array),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("average absolute deviation"),
                term_cursor::Down(2)
            );
        }
        standarddeviation!() => {
            println!(
                "{} {}",
                formuls::standard_deviation(array),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("standard deviation"),
                term_cursor::Down(2)
            );
        }
        linearcoefficient!() => {
            println!(
                "{} {}",
                formuls::linear_variation_coefficient(array),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("linear variation coefficient"),
                term_cursor::Down(2)
            );
        }
        standardcoefficient!() => {
            println!(
                "{} {}",
                formuls::standard_variation_coefficient(array),
                term_cursor::Up(2)
            );
            println!(
                "{} {}",
                Green.bold().paint("standard variation coefficient"),
                term_cursor::Down(2)
            );
        }
        variance!() => {
            println!("{} {}", formuls::variance(array), term_cursor::Up(2));
            println!(
                "{} {}",
                Green.bold().paint("variance"),
                term_cursor::Down(2)
            );
        }
        colmogorov!() => {
            println!("{}{}", formuls::mean_colmogorov(array), term_cursor::Up(2));
            println!(
                "{} {}",
                Green.bold().paint("mean colmogorov"),
                term_cursor::Down(2)
            );
        }
        help!() => {
            println!("{}{}", term_cursor::Up(1), Green.bold().paint("help"));
            print_man();
        }
        printdata!() => {
            for item in array {
                print!("{} ", item)
            }

            println!(
                "\n{}{}{}",
                term_cursor::Up(2),
                Green.bold().paint("print_data"),
                term_cursor::Down(1)
            );
        }

        _ => println!("{}", Red.paint("No valid argument\nTry 'help'")),
    }
}

pub fn scen_new_array(status: usize, array: &mut Vec<f64>) {
    match status {
        add!() => {
            println!("{}{}", term_cursor::Up(1), Green.bold().paint("add"));
            let new_array = input_array();
            for i in 0..new_array.len() {
                array.push(new_array[i]);
            }
            sorting(array);
        }

        addfile!() => {
            println!(
                "{}{}",
                term_cursor::Up(1),
                Green.bold().paint("add from file")
            );
            let new_array = input_array_file();
            for item in new_array {
                array.push(item);
            }
            sorting(array);
        }

        excludevalue!() => {
            println!(
                "{}{}",
                term_cursor::Up(1),
                Green.bold().paint("exclude number by value")
            );
            let numbers_exclude = input_array_usize();
            let mut i = 0;
            while i < array.len() {
                let mut check = 0;
                for u in 0..numbers_exclude.len() {
                    if numbers_exclude[u] as f64 == array[i] {
                        array.remove(i);
                        check = 1;
                    }
                }
                if check == 0 {
                    i += 1;
                }
            }
        }

        excludeindex!() => {
            println!(
                "{}{}",
                term_cursor::Up(1),
                Green.bold().paint("exclude number by index")
            );
            let numbers_exclude = input_array_usize();
            let mut check = 0;
            for i in 0..numbers_exclude.len() {
                if numbers_exclude[i] > array.len() || numbers_exclude[i] < 0 {
                    check = 1
                }
            }
            if check == 1 {
                println!("{}", Red.paint("Index out of range"))
            } else {
                for i in 0..numbers_exclude.len() {
                    array.remove(i);
                }
            }
        }

        excludemax!() => {
            println!(
                "{}{}",
                term_cursor::Up(1),
                Green.bold().paint("exclude maxes")
            );
            let mut i = 0;
            let min = array[array.len() - 1];
            while i < array.len() {
                if array[i] == min {
                    array.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        excludemin!() => {
            println!(
                "{}{}",
                term_cursor::Up(1),
                Green.bold().paint("exclude mins")
            );
            let mut i = 0;
            let max = array[0];
            while i < array.len() {
                if array[i] == max {
                    array.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        new!() => {
            println!("{}{}", term_cursor::Up(1), Green.bold().paint("new data"));
            for i in 0..array.len() {
                array.pop();
            }
            let array_to_realloc = input_array();
            for item in array_to_realloc {
                array.push(item);
            }
            sorting(array);
        }

        _ => println!("{}", Red.paint("No valid argument")),
    }
}
