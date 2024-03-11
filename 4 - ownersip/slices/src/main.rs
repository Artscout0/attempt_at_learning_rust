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
    let words = String::from("Hello, world!");

    println!("{}", first_word(&words));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}