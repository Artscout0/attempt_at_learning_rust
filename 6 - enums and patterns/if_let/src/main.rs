// If let is a more consise way of matching, 
// if you only want to do something in a specific case.

fn main() {
    // These do the exact same thing, but number two is more readeable.

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

// Use if let in cases where you need to check 
// for one specific thing, and match for multiple.