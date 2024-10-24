mod lib;
mod variants;
mod options;

use std::io;
fn main() {
    let mut array= options::input_array();
    options::sorting(&mut array);
    let mut status = 23;



    loop {
        status=23;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        for i in 0..23 {
            if line.trim() == variants::INPUT_OPTIONS[i] {
                status = i;
            }
        }
        if status==15 {break;}
        else if status>15 && status<22 { array=options::scen_new_array(status, &array);}
        else {options::scen(status, &array);}

        while array.is_empty() {
            println!("Empty array, input new one:");
            array=options::input_array();
        }

    }
}




