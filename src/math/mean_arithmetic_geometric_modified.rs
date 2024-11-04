/*модифицированное среднее арифметико-геометрическое*/
pub fn mean_arithmetic_geometric_modified(array: &[f64]) -> f64 {
    let mut a_array: Vec<f64> = Vec::new();
    let mut b_array: Vec<f64> = Vec::new();
    let mut c_array: Vec<f64> = Vec::new();
    a_array.push(array[0]);
    b_array.push(array[1]);
    c_array.push(0.0);
    let mut i: usize = 1;
    while b_array[i - 1] > 0.0 {
        a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
        b_array.push(
            c_array[i - 1]
                + ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1])).sqrt(),
        );
        c_array.push(
            c_array[i - 1]
                - ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1])).sqrt(),
        );
        i += 1;
    }
    a_array[i - 1]
}
