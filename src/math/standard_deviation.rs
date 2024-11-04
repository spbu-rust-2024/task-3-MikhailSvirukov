/*среднее квадратичное отклонение*/
use crate::math::mean_arithmetic::mean_arithmetic;

pub fn standard_deviation(array: &Vec<f64>) -> f64 {
    let average = mean_arithmetic(array);
    let mut sum = 0.0;
    let t = array.len() as f64;
    for elements in array {
        sum += (average - elements) * (average - elements);
    }
    (sum / t).sqrt()
}
