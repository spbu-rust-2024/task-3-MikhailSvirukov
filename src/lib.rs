use std::io;

/*среднее арифметическое*/
pub mod formuls {
    pub fn mean_arithmetic(array: &Vec<f64>) -> f64 {
        let r: f64 = array.len() as f64;
        let sum: f64 = array.iter().sum::<f64>();
        sum / r
    }
    /*среднее степенное*/
    pub fn mean_generalized(array: &Vec<f64>, d: f64) -> f64 {
        let r: f64 = array.len() as f64;
        let mut sum: f64 = 1.0;
        for elements in array {
            sum = sum + (elements.powf(d));
        }
        (sum / r).powf(1.0 / d)
    }
    /*среднее геометрическое*/
    pub fn mean_geometric(array: &Vec<f64>) -> f64 {
        let r: f64 = array.len() as f64;
        let mut sum: f64 = 0.0;
        for elements in array {
            sum += elements.powf(1.0 / r);
        }
        sum
    }
    /*среднее арифметико-геометрическое*/
    pub fn mean_arithmetic_geometric(array: &Vec<f64>, e: usize) -> f64 {
        let mut a_array: Vec<f64> = Vec::new();
        let mut b_array: Vec<f64> = Vec::new();
        a_array.push(array[0]);
        b_array.push(array[1]);
        let mut i: usize = 1;
        while i < e {
            a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
            b_array.push((a_array[i - 1] * b_array[i - 1]).sqrt());
            i = i + 1;
        }
        a_array[e - 1]
    }
    /*модифицированное среднее арифметико-геометрическое*/
    pub fn mean_arithmetic_geometric_modified(array: &Vec<f64>) -> f64 {
        let mut a_array: Vec<f64> = Vec::new();
        let mut b_array: Vec<f64> = Vec::new();
        let mut c_array: Vec<f64> = Vec::new();
        a_array.push(array[0]);
        b_array.push(array[1]);
        c_array.push(0.0);
        let mut i: usize = 1;
        while b_array[i - 1] > 0.0 {
            a_array.push((a_array[i - 1] + b_array[i - 1]) / 2.0);
            b_array.push(c_array[i - 1] + ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1])).sqrt());
            c_array.push(c_array[i - 1] - ((a_array[i - 1] - c_array[i - 1]) * (b_array[i - 1] - c_array[i - 1])).sqrt());
            i = i + 1;
        }
        a_array[i - 1]
    }
    /*среднее усеченное*/
    pub fn mean_truncated(array: &Vec<f64>, k: usize) -> f64 {
        let mut res: Vec<f64> = Vec::new();
        for i in k..array.len() - k {
            res.push(array[i]);
        }
        mean_arithmetic(&res)
    }
    /*винзоризованное среднее*/
    pub fn mean_winsorized(array: &Vec<f64>, k: usize) -> f64 {
        let mut res = array.clone();
        for i in 0..(k) {
            res[i] = res[k];
        }
        for i in (array.len() - k..array.len()) {
            res[i] = res[array.len() - k-1];
        }
        mean_arithmetic(&res)
    }
    /*медиана*/
    pub fn median(array: &Vec<f64>) -> f64 {
        if array.len() % 2 == 1 {
            array[array.len() / 2]
        } else {
            (array[array.len() /2] + array[array.len() / 2-1])/ 2.0
        }
    }
    /*мода*/


    //добавить макрос
    pub fn mode(array: &Vec<f64>) -> f64 {
        let mut max = 1;
        let mut count = 1;
        let mut numb = array[0];
        let mut dev = 1.0;
        for i in 1..array.len() {
            if array[i] != array[i - 1] {
                if max == count {
                    numb = numb + array[i];
                    dev = dev + 1.0
                } else if max < count {
                    numb = array[i - 1];
                    dev = 1.0;
                    max = count
                }
                count = 1;
            } else {
                count = count + 1;
            }
        }
        numb / dev
    }
    /*среднее линейное отклонение*/
    pub fn average_absolute_deviation(array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum = 0.0;
        let t = array.len() as f64;
        for elements in array {
            sum = sum + (average - (elements)).abs();
        }
        sum / t
    }
    /*среднее квадратичное отклонение*/
    pub fn standard_deviation(array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum = 0.0;
        let t = array.len() as f64;
        for elements in array {
            sum = sum + (average - elements) * (average - elements);
        }
        (sum / t).sqrt()
    }
    /*линейный коэффициент вариации*/
    pub fn linear_variation_coefficient(array: &Vec<f64>) -> f64{
        average_absolute_deviation(array)/mean_arithmetic(array)
    }
    /*квадратический коэффициент вариации*/
    pub fn standard_variation_coefficient(array: &Vec<f64>) -> f64{
        standard_deviation(array)/mean_arithmetic(array)
    }
    /*дисперсия*/
    pub fn variance (array: &Vec<f64>) -> f64 {
        let average = mean_arithmetic(array);
        let mut sum=0.0;
        let t=array.len() as f64 - 1.0;
        for elements in array {
            sum=sum+(average-elements)*(average-elements);
        }
        (sum/t).sqrt()
    }

}
