fn main() {
    // If you wat to create an empty vector, you'll have to 
    // provide the data type that's stored in it.
    // let v: Vec<i32> = Vec::new();

    // You can do it without specifying the type if you instead use vec![data];
    // let v = vec![1, 2, 3];


    // You add data to it by pushing it onto the end.
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    // You read data either using indexing, or using get
    // Indexing works well in a loop, get when you don't know the size of the array.

    let third1: &i32 = &v[2];
    println!("nb is: {}", third1);

    let third2: Option<&i32> = v.get(2);
    match third2 {
        Some(third) => println!("3rd element is {}", third),
        None => println!("There is no third")
    }

    // Indexing also starts at 0

    //iteration:

    for i in &v {
        println!("{}", i)
    }
}
