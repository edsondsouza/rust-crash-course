// fn main() {
//     // let tup = (500, 60, 1);

//     // let (x, y, z) = tup;

//     // println!("The value of x is: {x}");
//     // println!("The value of y is: {y}");
//     // println!("The value of z is: {z}");

//     let x = (200, 2.6, -1);

//     let first_tuple_index = x.0;
//     let second_tuple_index = x.1;
//     let third_tuple_index = x.2;

//     // print macha
//     println!("the value of first index is {first_tuple_index}");
//     println!("the value of second index is {second_tuple_index}");
//     println!("the value of third index is {third_tuple_index}");
// }

// early access of array

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
