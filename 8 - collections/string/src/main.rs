
fn main() {
    let s: &str = "Hello, world!";

    println!("{}", &s[0..2]);

    // this line is invalid, because Rust stores string weirdly, to allow for storage of unicode chars. This can ba annoying.
    // println!("{}", &s[0])

    // but you can still iterate over strings like this, so it isn't a problem most of the time.
    for c in s.chars() {
        println!("{}", c)
    }
}
