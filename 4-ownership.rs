// Ownership

// C, C++ has Memory Management Control Issues
// Garbage Collector solved this issue, but created a new issue ->
// Slow Performance: [stopping/resuming the program]

// Ownership introduced by Rust to solve memory safety isuues and high performance at the same time.
// Ownerhship Rules:
// 1. Every value in Rust has a variable that is its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Example 1: Each value in Rust has a variable that is its owner
// fn main() {
//     let s1 = String::from("rust");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example 2: There can only be one owner at a time
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // ownership of the value is moved from s1 to s2

//     // println!("{}", s1); // error: borrow of moved value: `s1`
//     println!("{}", s2); // valid
// }

// Example 3: When the owner goes out of scope, the value will be dropped
// fn main() {
//     {
//         let s = String::from("hello");
//         println!("{}", s);
//     } // s goes out of scope here and the string value is dropped

//     // println!("{}", s); // error: borrow of moved value: `s`
// }


fn main() {
    let s1 = String::from("rust");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn printLost(s: &String) {
    println!("{}", &s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}