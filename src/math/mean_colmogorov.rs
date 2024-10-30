pub fn mean_colmogorov(array: &Vec<f64>) -> f64 {
    let mut result = 0.0;
    for item in array {
        result += colmogorov(*item);
    }
    result /= array.len() as f64;
    let mut start: f64 = 0.0;
    let mut func_res = 0.0;
    while func_res < result {
        start += 0.125;
        func_res = colmogorov(start);
    }
    start
}

fn colmogorov(number: f64) -> f64 {
    number.powf(std::f64::consts::E) + number * number * 2.0 + number + 5.0
}