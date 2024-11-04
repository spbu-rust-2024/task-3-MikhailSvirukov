use crate::{math, Args};
use ansi_term::Color::{Green, Red};

pub fn varint(res: Args, array: Vec<f64>) {
    if res.mean_arithmetic {
        println!("{}", Green.bold().paint("mean arithmetic"));
        println!("{}", math::mean_arithmetic::mean_arithmetic(&array));
    }
    if res.mean_geometric {
        println!("{}", Green.bold().paint("mean geometric"));
        println!("{}", math::mean_geometric::mean_geometric(&array));
    }
    if res.median {
        println!("{}", Green.bold().paint("median"));
        println!("{}", math::median::median(&array));
    }
    if res.mode {
        println!("{}", Green.bold().paint("mode"));
        println!("{}", math::mode::mode(&array));
    }
    if res.mean_colmogorov {
        println!("{}", Green.bold().paint("mean colmogorov"));
        println!("{}", math::mean_colmogorov::mean_colmogorov(&array));
    }
    if res.variance {
        println!("{}", Green.bold().paint("variance"));
        println!("{}", math::variance::variance(&array));
    }
    if res.standard_variation_coefficient {
        println!("{}", Green.bold().paint("standard variation coefficient"));
        println!(
            "{}",
            math::standard_variation_coefficient::standard_variation_coefficient(&array)
        );
    }
    if res.linear_variation_coefficient {
        println!("{}", Green.bold().paint("linear variation coefficient"));
        println!(
            "{}",
            math::linear_variation_coefficient::linear_variation_coefficient(&array)
        );
    }
    if res.standard_deviation {
        println!("{}", Green.bold().paint("standard deviation"));
        println!("{}", math::standard_deviation::standard_deviation(&array));
    }
    if res.average_absolute_deviation {
        println!("{}", Green.bold().paint("average absolute deviation"));
        println!(
            "{}",
            math::average_absolute_deviation::average_absolute_deviation(&array)
        );
    }
    if res.mean_arithmetic_geometric_modified {
        println!(
            "{}",
            Green.bold().paint("mean_arithmetic_geometric_modified")
        );
        if array.len() == 2 {
            println!(
                "{}",
                math::mean_arithmetic_geometric_modified::mean_arithmetic_geometric_modified(
                    &array
                )
            );
        } else {
            println!("{}", Red.bold().paint("two values required"));
        }
    }

    if res.mean_truncated.is_some() {
        println!("{}", Green.bold().paint("mean truncated"));
        let numb = res.mean_truncated.unwrap();
        if numb * 2 > array.len() {
            println!("{}", Red.bold().paint("can not exclude that many numbers"));
        } else {
            println!("{}", math::mean_truncated::mean_truncated(&array, numb));
        }
    }

    if res.mean_winsorized.is_some() {
        println!("{}", Green.bold().paint("mean winsorized"));
        let numb = res.mean_winsorized.unwrap();
        if numb * 2 > array.len() {
            println!("{}", Red.bold().paint("can not replace that many numbers"));
        } else {
            println!("{}", math::mean_winsorized::mean_winsorized(&array, numb));
        }
    }

    if res.mean_generalized.is_some() {
        println!("{}", Green.bold().paint("mean generalized"));
        let numb = res.mean_generalized.unwrap();
        println!(
            "{}",
            math::mean_generalized::mean_generalized(&array, numb as f64)
        );
    }

    if res.mean_arithmetic_geometric.is_some() {
        println!("{}", Green.bold().paint("mean arithmetic geometric"));
        let numb = res.mean_arithmetic_geometric.unwrap();
        if numb == 2 {
            println!(
                "{}",
                math::mean_arithmetic_geometric::mean_arithmetic_geometric(&array, numb)
            );
        } else {
            println!("{}", Red.bold().paint("two values required"));
        }
    }
}
