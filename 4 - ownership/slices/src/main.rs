/*
a bit to write here:

first:

&str is a reference to a string litteral somewhere in the binary.

This means, that it's predefined and can't be changed.

Finally, I know why there is 2 different types of strings!

due to various reasons, it works as if it was a slice as well. I don't quite think I understand that, but I think I partially do.

Next, I need to explain what a slice is. 

A slice is a part of something. It can either be a string or an array.

that's about it.

*/

fn main() {
    let words = String::from("Hello, world!"); // Takes the &str of "Hello, world!", and creates a String with it, that you can work with. 

    let first_word = first_word(&words); // Pass the reference to word into the first_word function. 

    println!("{}", first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // gets the bytes of the string

    for (i, &item) in bytes.iter().enumerate(){ // iterate through the bytes
        if item == b' ' {
            return &s[0..i]; // if the current byte is equal to that of space, return a slice of the values from the first one until the current one.
        }
    }

    &s[..]
}