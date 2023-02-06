use std::io;

fn main() {
    let x = 2.0; // f64, default
    let y: f32 = 3.0; // f32
    println!("x:{x}, y:{y}");

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("product: {product}");
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1
    println!("quotient: {quotient}, truncated: {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // 4 bytes->Unicode
    println!("c:{c}, z:{z}, heart_eyed_cat:{heart_eyed_cat}");

    let tup = (500, 6.4, 1); // tuple
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let x = tup.0; // index by .
    let z = tup.2;
    println!("x:{x}, z:{z}");


    let a = [1, 2, 3, 4, 5]; // Array

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
