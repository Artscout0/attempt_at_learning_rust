use std::fs::File;
use std::io::{self, Read};

// the idea of propagating errors is:
// you have an error?
// return it.
// that way all errors are handled in main(), or as close to it as needed.
// this is one way to do it.

/*
fn main() {
    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(error) => println!("Error! contents: {}", error),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/

// a better way to do it is using the ? operator, which when placed after a Result<[type], Error> 
// proagates the error, or returns the result if there isn't an error.
// also, ? can be chained. 
// be careful, ? can't be used in main(), you'll have to handle your errors head on.
// Well, unless you make it so main() -> Result<(), Box<dyn Error>>, which would mean that main can either return nothing or
// any error.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(error) => println!("Error! contents: {}", error),
    }
}

// ? can also be used with Option<T>, but instead of an error it'll be a none.