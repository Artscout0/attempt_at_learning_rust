use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // You often will encounter potential errors, such as here, as hello.txt doesn't exist.
    // This code will create a the file if it doesn't find it. 
    // as you can see here, most if not all of these cases in Rust instead of returning only a specific value 
    // or making an error, instead return a Result<[type], Error>. Using that, you can handle anything as you want, 
    // by just checking if the code was successful or not.
    // You can also easily handle different errors differently, by just using std::io::ErrorKind, which allows you to classify errors
    // like in the example. 

    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let buf: &mut Vec<u8> = &mut vec![]; 
    let res = greeting_file.by_ref().read_to_end(buf);

    match res {
        Ok(size) => println!("result of size {size} {:?}", buf),
        Err(err) => println!("Error reading file: {}", err)
    }
    
}