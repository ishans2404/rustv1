// Control Flow 

// If Else

#![allow(warnings)]
fn main() {
    // let age = 16;
    // if age >= 18 {
    //     println!("you can drive a car");
    // } else {
    //     println!("you cannot drive a car");
    // }

    // multiple conditions
    // let num = 6;
    // if num % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if num % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if num % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3 or 2");
    // }

    // Using if in let statement
    let mut condition = true;
    let mut res = if condition {5} else {6};
    println!("res value: {}", res);
    condition = false;
    res = if condition {5} else {6};
    println!("new res value: {}", res);

}