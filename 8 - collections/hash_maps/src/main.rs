use std::collections::HashMap;

fn main() {
    // HashMaps are associative arrays in other langs.
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("blue"), 10);
    map.insert(String::from("yellow"), 20);

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // as in most cases, stuff like String will transfer ownership, integers and similar won't.

    // a subsequent insert will override the previous one
    map.insert(String::from("Blue"), 15);
    println!("{:?}", map);

    // if you only want to add, not override, you'll need to get an entry of that key, and if it's empty instert on it.

    map.entry(String::from("blue")).or_insert(20);
    map.entry(String::from("red")).or_insert(5);
    println!("{:?}", map);

    // notice how all of these outputs have different random orders? Yeah it does that.

    // You can also make it use it's own hasher/hash function, but I'm a bit too bad at coding in this language to do so. 
    // I'll do an example one once I'm good enough.
}