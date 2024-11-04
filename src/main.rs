use ansi_term::Color::{Green, Red};
use clap::Parser;
use std::fs;
use std::ops::Not;

mod math;
mod side_funcs;
mod variants;

#[derive(Parser, Debug)]

struct Args {
    #[arg(short = 'f', long = "file", required = true)]
    file: String,
    #[arg(long = "mean_arithmetic")]
    mean_arithmetic: bool,

    #[arg(long = "mean_geometric")]
    mean_geometric: bool,

    #[arg(long = "mean_arithmetic_geometric_modified")]
    mean_arithmetic_geometric_modified: bool,

    #[arg(long = "mean_truncated")]
    mean_truncated: Option<usize>,

    #[arg(long = "mean_winsorized")]
    mean_winsorized: Option<usize>,

    #[arg(long = "mean_generalized")]
    mean_generalized: Option<usize>,

    #[arg(long = "mean_arithmetic_geometric")]
    mean_arithmetic_geometric: Option<usize>,

    #[arg(long = "median")]
    median: bool,
    #[arg(long = "mode")]
    mode: bool,
    #[arg(long = "average_absolute_deviation")]
    average_absolute_deviation: bool,
    #[arg(long = "standard_deviation")]
    standard_deviation: bool,
    #[arg(long = "linear_variation_coefficient")]
    linear_variation_coefficient: bool,
    #[arg(long = "standard_variation_coefficient")]
    standard_variation_coefficient: bool,
    #[arg(long = "variance")]
    variance: bool,
    #[arg(long = "mean_colmogorov")]
    mean_colmogorov: bool,
}

fn main() {
    let res = Args::parse();
    let line = res.file.as_str();
    let mut array = side_funcs::input_array_file(line);
    side_funcs::sorting(&mut array);
    variants::varint(res, array);
}
