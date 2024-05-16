use std::fs::File;

fn main() {
    // .unwrap can make life easier, as it just makes this all much simpler.
    let greeting_file = File::open("hello.txt").unwrap();
}