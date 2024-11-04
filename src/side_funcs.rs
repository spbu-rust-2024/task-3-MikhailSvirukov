use std::fs;

pub fn input_array_file(name: &str) -> Vec<f64> {
    let line = fs::read_to_string(name.trim()).expect("Should have been able to read the file");
    let array: Vec<f64> = line
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().expect("not a number"))
        .collect();
    array
}

pub fn sorting(array: &mut [f64]) {
    let mut counter = 1;
    while counter < array.len() {
        let mut i = counter;
        while i > 0 {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
            }
            i -= 1;
        }
        counter += 1;
    }
}
