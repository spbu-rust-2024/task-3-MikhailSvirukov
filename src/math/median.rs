/*медиана*/
pub fn median(array: &[f64]) -> f64 {
    if array.len() % 2 == 1 {
        array[array.len() / 2]
    } else {
        (array[array.len() / 2] + array[array.len() / 2 - 1]) / 2.0
    }
}
