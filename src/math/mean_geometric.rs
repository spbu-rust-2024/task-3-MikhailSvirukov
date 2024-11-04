/*среднее геометрическое*/
pub fn mean_geometric(array: &Vec<f64>) -> f64 {
    let r: f64 = array.len() as f64;
    let mut sum: f64 = 1.0;
    for elements in array {
        sum *= elements;
    }
    sum.powf(1.0 / r)
}
