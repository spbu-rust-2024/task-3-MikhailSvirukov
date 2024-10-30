/*линейный коэффициент вариации*/
use crate::math::mean_arithmetic::mean_arithmetic;
use crate::math::average_absolute_deviation::average_absolute_deviation;

pub fn linear_variation_coefficient(array: &Vec<f64>) -> f64 {
    average_absolute_deviation(array) / mean_arithmetic(array)
}