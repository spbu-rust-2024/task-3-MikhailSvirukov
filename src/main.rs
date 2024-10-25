mod lib;
mod variants;
mod options;
use std::io;
use std::fs;



fn main() {
    println!("Choose input variant:\n--file\n--print");
    let mut input_option:String=String::new();
    let mut array:Vec<f64>=Vec::new();
    io::stdin()
        .read_line(&mut input_option)
        .expect("Failed to read line");
    match input_option.as_str() {
        "print\n" =>  array = options::input_array(),
        "file\n" => array= options::input_array_file(),
        _ => {println!("No such option"); return;}
    };
    options::sorting(&mut array);
    let mut status = 24;

    loop {
        status=24;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        for i in 0..24 {
            if line.trim() == variants::INPUT_OPTIONS[i] {
                status = i;
            }
        }
        if status==15 {break;}
        else if status>15 && status<23 { array=options::scen_new_array(status, &array);}
        else {options::scen(status, &array);}

        while array.is_empty() {
            println!("Empty array, input new one:");
            array=options::input_array();
        }
    }
}




