/*винзоризованное среднее*/
use crate::math::mean_arithmetic::mean_arithmetic;

pub fn mean_winsorized(array: &[f64], k: usize) -> f64 {
    let mut res = array.to_owned();
    for i in 0..k {
        res[i] = res[k];
    }
    for i in array.len() - k..array.len() {
        res[i] = res[array.len() - k - 1];
    }
    mean_arithmetic(&res)
}