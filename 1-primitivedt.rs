// Primitive data types
// int, float, bool, char

fn main() {
// Integer
// Rust has signed (+  and -) and unsigned integers
// i8, i16, i32, i64, i128: signed integers
// u8, u16, u32, u64, u128: unsigned integers
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signed integer: {}", x);
    println!("unsigned integer: {}", y);

// Floats
// f32, f64
    let pi: f64 = 3.14;
    println!("value of pi: {}", pi);

// Booleans
// true, false
    let flag: bool = true;
    println!("value of flag: {}", flag);

// Character type
    let ch: char = 'a';
    println!("value of ch: {}", ch);
}