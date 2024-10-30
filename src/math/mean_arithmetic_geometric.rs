/*среднее арифметико-геометрическое*/
pub fn mean_arithmetic_geometric(array: &[f64], e: usize) -> f64 {
    let mut a_array: Vec<f64> = Vec::new();
    let mut b_array: Vec<f64> = Vec::new();
    a_array.push(array[0]);
    b_array.push(array[1]);
    let mut i: usize = 1;
    while i < e {
        a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
        b_array.push((a_array[i - 1] * b_array[i - 1]).sqrt());
        i += 1;
    }
    a_array[e - 1]
}