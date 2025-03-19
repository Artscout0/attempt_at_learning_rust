fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
so, if you use a reference (&), the ref goes out of scope, but doesn't get 
dropped because the reference has no ownership over the value.

this is called borrowing.
*/
