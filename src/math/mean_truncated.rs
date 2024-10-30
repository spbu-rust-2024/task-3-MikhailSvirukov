/*среднее усеченное*/
use crate::math::mean_arithmetic::mean_arithmetic;

pub fn mean_truncated(array: &[f64], k: usize) -> f64 {
    let mut res: Vec<f64> = Vec::new();
    for item in array.iter().take(array.len() - k).skip(k) {
        res.push(*item);
    }
    mean_arithmetic(&res)
}