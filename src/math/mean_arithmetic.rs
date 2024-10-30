/*среднее арифметическое*/
pub fn mean_arithmetic(array: &[f64]) -> f64 {
    let r: f64 = array.len() as f64;
    let sum: f64 = array.iter().sum::<f64>();
    sum / r
}