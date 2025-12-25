// Error Handling (2 approaches)

// // Approach 1
// enum Option<T>{ // Defining the generic Option Type
//     Some(T), // Represents a value
//     None, // Represents no value
// }

// // Approach 2
// enum Result<T, E> { // Define the generic Result type
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }

// // Approach 1
// fn divideOption(numerator: f64, denominator: f64) -> Option<f64>{
//     if denominator == 0.0 {
//         None
//     } else {
//         Some(numerator / denominator)
//     }
// }

// Approach 2
fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
        Err("cannot divide by zero lmao".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {

    // let res = divideOption(10.0, 0.0);
    // match res{
    //     Some(x) => println!("Result: {}", x),
    //     None => println!("Cannot divide by zero lol"),
    // }

    // let res = divideOption(10.0, 3.0);
    // match res{
    //     Some(x) => println!("Result: {}", x),
    //     None => println!("Cannot divide by zero lol"),
    // }

    // let res = divideResult(12.0, 3.0);
    // match res{
    //     Ok(x) => println!("Result: {}", x),
    //     Err(e) => println!("Error: {}", e),
    // }

    let res = divideResult(12.0, 0.0);
    match res{
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

}