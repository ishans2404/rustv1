// Functions
// main fn is the entry point of every rust program
// any function / variable should be defined in snake case
// snake case: hello_world
// camel case: helloWorld
// kebab case: hello-world
// use "const" or "static" keywords instead of "let" for global variables


fn main() {
    println!("hello rust");
    hello_world();
    tell_age(25);
    human_id("Alice", 30, 165.0);

    let _X = {
        let price = 5;
        let quantity = 10;
        price * quantity
    };
    println!("\nTotal cost is: {}", _X);

    let y = add(15, 25);
    println!("\nvalue of y is: {}", y);
    println!("value from function 'add' is: {}", add(15, 25));

    let weight = 65.0;
    let height = 1.75;
    let bmi = calculate_bmi(weight, height);
    println!("\nYour BMI is: {:.2}", bmi);
}

// hoisting: functions can be called before they are defined
fn hello_world() {
    println!("hello world");
}

// you can insert input values
fn tell_age(age: u32) {
    println!("\ni am {} years old.", age);
}

// insert more than 1 values
fn human_id(name: &str, age: u32, height: f32) {
    println!("\nMy name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Expressions vs Statements
// Expression: anything that returns a value
// 5
// true & false
// add(3, 4)
// if condition {value1} else {value2}
// ({code}) this is a code block

// Statement: anything that does not return a value
// almost all statements in rust end with ;
// let y = let x = 5;
// 1 variable declarations: let x = 5;
// 2 function definitions: fn foo() {}
// 3 control flow statements: if condition { ... } else { ... },
//   loop { ... }, while condition { ... }, for item in collection { ... }, etc.
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}