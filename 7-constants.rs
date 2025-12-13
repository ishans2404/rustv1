// Constants

fn main() {
    // we cannot use "mut" decorator with constants
    let mut x = 5;
    // const mut y = 10; // error: 'mut' not allowed with const

    // use capital letters with underscores and type annotation for constants
    const Y: i32 = 10;

    println!("value of x: {}", x);
    println!("value of constant Y: {}", Y);
    println!("value of constant PI: {}", PI);
}

// global constant
const PI: f64 = 3.14159;