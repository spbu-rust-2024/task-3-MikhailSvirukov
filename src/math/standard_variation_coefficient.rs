/*квадратический коэффициент вариации*/
use crate::math::mean_arithmetic::mean_arithmetic;
use crate::math::standard_deviation::standard_deviation;

pub fn standard_variation_coefficient(array: &Vec<f64>) -> f64 {
    standard_deviation(array) / mean_arithmetic(array)
}
