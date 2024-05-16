// Here we have two functions that work reasonably well. 
// The issue is, they both do the exact same thing.
/*
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// This can be rewritten as follows:
fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T { // the std::cmp::PartialOrd is a Trait. Covered soon. 
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Generics can alo be used in struct definitions, enum definitions, and as mentioned previously method definitions.