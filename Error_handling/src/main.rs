//packages 

use std::io::ErrorKind;
use std::fs::File;

//Main Function

fn main() {
    println!("Hello, world!, Welcome to error handling program");
    let file_handling_result = File::open("option.txt");
    let file_handled = match file_handling_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match  File::create("option.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {e:?}"),
            },  
        _ => {
            panic!("Problem creating file {error:?}")
        }
    },
    };
}

