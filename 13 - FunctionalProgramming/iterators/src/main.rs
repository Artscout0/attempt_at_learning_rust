// Do you know C#'s linq? this is the same overall idea.
// And I only said C# cause it's one of my main langs, I 
// know a lot of languages have something similar.
// Still like how it's implemented here.

fn main() {

    // iter() allows you to iterate over something.
    // langs that don't have iterators, such as again C#, would require 
    // you to do so using indexes and such.
    // This isn't inherently better, but it can be neater, and often imo looks tidier.
    /*
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
    */

    // Iterators do this all themselves, by implementing the Iterator trait. 
    // The trait makes it so something requires the next(&mut self) -> Option(Self::Item) method (self:: item being the type)
    // note that self is mutable, as it requires self to store data about what was the last selected item. 
    
    // Basically, iteration works by calling next(), and giving you the result.
    // See test 1

    // Some funcs take control of the iterator, and do stuff with the result
    // See test 2

    // Some funcs take control of the iterator, and return a new one.
    /*
    let v1: Vec<i32> = vec![1, 2, 3];
    
    v1.iter().map(|x| x + 1); // "iterators are lazy and do nothing unless consumed" is arguably my favorite ever thing to have read come out as an warning/error message
    
    println!("{:?}", v1);
    */
    // an implementation that has no problems:
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v1);
    println!("{:?}", v2);
    // map iterates through an iterator, applies a closure to each element in it, and returns a new iterator 
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// Example of a function that uses the iterator. 
// .sum() works by repeatedly calling .next() until it is None (see previous example), and adding the result.
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}