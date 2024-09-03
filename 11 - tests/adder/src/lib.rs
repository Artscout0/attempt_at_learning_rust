/**
 * Tests are neat. You can basically use them to test if functions within your code function as expected. 
 * For example, you can test if the result of some function with the given params results in the wanted result.
 * 
 * It could be done using just println!(), but using the built in tests does make it a bit simpler. 
 * Now to remember that it exists and actually use it...
 */


// all this is setup
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn new_other(value: i32) -> Guess {
        if value < 1 { // intentional error
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}
// setup ends


#[cfg(test)]
mod tests {
    use super::*;

    // successful test
    #[test]
    fn add_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // test fails, because panic
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    //fails, because assert_eq fails
    #[test]
    fn add_not_two() {
        let result = add(2, 3);
        assert_eq!(result, 4);
    }

    // how to test functions withing structs
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // if something should panic, use #[should_panic] to indicate that if it panics it is successful, if it doesn't it fails.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn greater_than_100_other() {
        Guess::new_other(200);
    }

    // should panic can expect something specific about the panic, that way it passes only if it panics because of something specific.
    // syntax:
    // #[should_panic(expected = "part of expected error")]

    // Now, obviously panicing isn't always the solution, and most of the time the better idea is (or at least to me seems to be) to return an Option<T, E> 
    // And it's implemented pretty much as expected
    // In this case it does have the drawback of not working with should_panic(expected) though, so I'll likely test 
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}