// match allows you to check if an enum has what you would expect.
//
// it works somewhat like a switch case:
//      it goes through every branch, and if its equal to 
//      the branch's value it does something.
// 
// each branch can execute a different usually anonym function, 
// like return 5, or println!("smth")

#[derive(Debug)] // you can also derive the debug of enums.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn part1() {
    let val: Coin = Coin::Penny;

    println!("{:#?}", val);

    println!("value: {}", value_in_cents(val));
}

// You can also make an enum hold data from another enum in it.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska, 
    // ETC...
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn part2() {
    let val = Coin2::Quarter(UsState::Alabama);

    println!("{}", value_in_cents2(val));
}

fn main(){
    part1();
    part2();
}

// you can also use the other keyword to math anything. 
// It's the equivalent of default in a switch case.

// If you don't need to use the other value, you can replace it with _

