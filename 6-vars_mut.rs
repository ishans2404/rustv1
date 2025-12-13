// Variables and Mutability

fn main() {
    // By default, variables are immutable in Rust
    // let a = 5;
    // println!("value of a: {}", a);
    // a = 10; // error: cannot assign twice to immutable variable

    // To make a variable mutable, use the "mut" keyword
    let mut a = 5;
    println!("value of a: {}", a);
    a = 10;
    println!("new value of a: {}", a);
}