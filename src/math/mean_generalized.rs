pub fn mean_generalized(array: &Vec<f64>, d: f64) -> f64 {
    let r: f64 = array.len() as f64;
    let mut sum: f64 = 1.0;
    for elements in array {
        sum += elements.powf(d);
    }
    (sum / r).powf(1.0 / d)
}