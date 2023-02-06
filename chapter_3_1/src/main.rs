// constant
const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("The constant value is: {THREE_HOURS_IN_SECOND}");

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("spaces_str: {spaces}");
    let spaces = spaces.len();
    println!("spaces_num: {spaces}");
}
