/*
    now for integers it works quite differently from strings.
    That is because we actually know the size of integers, as opposed to strings
 */

fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y)
}

/*
    This also applies to:
     - integers,
     - booleans
     - floats
     - chars
     - tuples made of the previously mentioned data
*/

/*
    as for functions, if you pass a function a string, it will move the data, 
    but an int will copy it.
*/

/*
    basically, work with the assumption that if I use one of the datatypes of set length, 
    I can use it after giving it's value, but not of unset value.
 */

 