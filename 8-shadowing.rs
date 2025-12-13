// Shadowing

// Shadowing is not the same as marking a variable as mutable with `mut`.

fn main() {
    let x = 5;
    let x = x + 1; // Shadowing the previous `x`

    {
        let x = x * 2; // Shadowing again in a new scope
        println!("The value of x in the inner scope is: {}", x); // Outputs 12
    }

    println!("The value of x in the outer scope is: {}", x); // Outputs 6
}