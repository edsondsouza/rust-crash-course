fn main() {
    // let mut x = 5; // x is mutable
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5; // x is immutable

    let x = x + 1; // shadowing

    {
        let x = x * 2; // shadowing
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}

// variables immutability
// constants
// shadowing
