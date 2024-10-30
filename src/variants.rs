pub mod var {
    pub const OPTION_COUNT: usize = INPUT_OPTIONS.len();

    pub const INPUT_OPTIONS: [&str; 26] = [
        "mean_arithmetic",
        "mean_generalized",
        "mean_geometric",
        "mean_arithmetic_geometric",
        "mean_arithmetic_geometric_modified",
        "mean_truncated",
        "mean_winsorized",
        "median",
        "mode",
        "average_absolute_deviation",
        "standard_deviation",
        "linear_variation_coefficient",
        "standard_variation_coefficient",
        "variance",
        "mean_colmogorov",
        "help",
        "exit",
        "add",
        "add_from_file",
        "exclude_number_by_value",
        "exclude_number_by_index",
        "exclude_maxes",
        "exclude_mins",
        "new_data",
        "print_data",
        "invalid"
    ];
}
