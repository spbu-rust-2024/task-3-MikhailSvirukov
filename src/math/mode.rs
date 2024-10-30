


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