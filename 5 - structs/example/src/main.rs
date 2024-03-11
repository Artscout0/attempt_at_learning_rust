/*
// Works, but bad practice as we can't easily know that width and height are related.
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/*
// better, because it's more clean, but worse, as tuples don't name their elements. 
fn main(){
    let rect1: (u32, u32) = (30, 50);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimentions:(u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}
*/



// best, as we have the name of the values in the struct, as well as only one element to give to the func.
#[derive(Debug)] // this notation allows println! to output this, although it must be given in {:?} (or {:#?}), as this is debug.
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}
