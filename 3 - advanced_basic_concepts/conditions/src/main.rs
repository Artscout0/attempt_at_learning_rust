fn main() {
    let condition = true;

    /* let number = if condition { 5 } else { "six" }; */ //doesn't work becuse expects a numeric answer

    let number = if condition { 5 } else { 6 }; // Works, because both are integers

    println!("The value of number is: {number}");
}