/*среднее линейное отклонение*/
use crate::math::mean_arithmetic::mean_arithmetic;

pub fn average_absolute_deviation(array: &Vec<f64>) -> f64 {
    let average = mean_arithmetic(array);
    let mut sum = 0.0;
    let t = array.len() as f64;
    for elements in array {
        sum += (average - (elements)).abs();
    }
    sum / t
}
