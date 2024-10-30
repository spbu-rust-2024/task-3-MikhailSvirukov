pub mod formuls {

    /*среднее арифметическое*/
    pub fn mean_arithmetic(array: &[f64]) -> f64 {
        let r: f64 = array.len() as f64;
        let sum: f64 = array.iter().sum::<f64>();
        sum / r
    }
    /*среднее степенное*/
    pub fn mean_generalized(array: &Vec<f64>, d: f64) -> f64 {
        let r: f64 = array.len() as f64;
        let mut sum: f64 = 1.0;
        for elements in array {
            sum += elements.powf(d);
        }
        (sum / r).powf(1.0 / d)
    }
    /*среднее геометрическое*/
    pub fn mean_geometric(array: &Vec<f64>) -> f64 {
        let r: f64 = array.len() as f64;
        let mut sum: f64 = 1.0;
        for elements in array {
            sum *= elements;
        }
        sum.powf(1.0 / r)
    }
    /*среднее арифметико-геометрическое*/
    pub fn mean_arithmetic_geometric(array: &[f64], e: usize) -> f64 {
        let mut a_array: Vec<f64> = Vec::new();
        let mut b_array: Vec<f64> = Vec::new();
        a_array.push(array[0]);
        b_array.push(array[1]);
        let mut i: usize = 1;
        while i < e {
            a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
            b_array.push((a_array[i - 1] * b_array[i - 1]).sqrt());
            i += 1;
        }
        a_array[e - 1]
    }
    /*модифицированное среднее арифметико-геометрическое*/
    pub fn mean_arithmetic_geometric_modified(array: &[f64]) -> f64 {
        let mut a_array: Vec<f64> = Vec::new();
        let mut b_array: Vec<f64> = Vec::new();
        let mut c_array: Vec<f64> = Vec::new();
        a_array.push(array[0]);
        b_array.push(array[1]);
        c_array.push(0.0);
        let mut i: usize = 1;
        while b_array[i - 1] > 0.0 {
            a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
            b_array.push(
                c_array[i - 1]
                    + ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1]))
                        .sqrt(),
            );
            c_array.push(
                c_array[i - 1]
                    - ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1]))
                        .sqrt(),
            );
            i += 1;
        }
        a_array[i - 1]
    }
    /*среднее усеченное*/
    pub fn mean_truncated(array: &[f64], k: usize) -> f64 {
        let mut res: Vec<f64> = Vec::new();
        for item in array.iter().take(array.len() - k).skip(k) {
            res.push(*item);
        }
        mean_arithmetic(&res)
    }
    /*винзоризованное среднее*/
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
    /*медиана*/
    pub fn median(array: &[f64]) -> f64 {
        if array.len() % 2 == 1 {
            array[array.len() / 2]
        } else {
            (array[array.len() / 2] + array[array.len() / 2 - 1]) / 2.0
        }
    }
    /*мода*/
    pub fn mode(array: &[f64]) -> f64 {
        let mut max = 0;
        let mut count = 1;
        let mut numb = 0.0;
        let mut dev = 1.0;
        for i in 1..array.len() {
            if array[i] != array[i - 1] {
                if max == count {
                    numb += array[i - 1];
                    dev += 1.0;
                } else if max < count {
                    numb = array[i - 1];
                    dev = 1.0;
                    max = count;
                }
                count = 1;
            } else {
                count += 1;
            }
        }
        if max == count {
            numb += array[array.len() - 1];
            dev += 1.0;
        } else if max < count {
            numb = array[array.len() - 1];
            dev = 1.0;
        }
        numb / dev
    }
    /*среднее линейное отклонение*/
    pub fn average_absolute_deviation(array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum = 0.0;
        let t = array.len() as f64;
        for elements in array {
            sum += (average - (elements)).abs();
        }
        sum / t
    }
    /*среднее квадратичное отклонение*/
    pub fn standard_deviation(array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum = 0.0;
        let t = array.len() as f64;
        for elements in array {
            sum += (average - elements) * (average - elements);
        }
        (sum / t).sqrt()
    }
    /*линейный коэффициент вариации*/
    pub fn linear_variation_coefficient(array: &Vec<f64>) -> f64 {
        average_absolute_deviation(array) / mean_arithmetic(array)
    }
    /*квадратический коэффициент вариации*/
    pub fn standard_variation_coefficient(array: &Vec<f64>) -> f64 {
        standard_deviation(array) / mean_arithmetic(array)
    }
    /*дисперсия*/
    pub fn variance(array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum = 0.0;
        let t = array.len() as f64 - 1.0;
        for elements in array {
            sum += (average - elements) * (average - elements);
        }
        (sum / t).sqrt()
    }

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
}
