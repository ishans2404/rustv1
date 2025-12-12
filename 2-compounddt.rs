// Compound Data Types
// arrays, tuples, slices and strings

// all datatypes are immutable by default in rust,
// add "mut" decorator while initializing the variable to make it mutable

fn main() {
// Arrays
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers array: {:?}", nums);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("fruits array: {:?}", fruits);
    println!("fruits array 1st element: {}", fruits[0]);

// Tuples
    let tuple1 = ("alice", 30, false);
    println!("tuple1 tuple: {:?}", tuple1);

    let tuple2: (&str, i32, bool) = ("sam", 45, true);
    println!("tuple2 tuple: {:?}", tuple2);

    let tuple3: (String, i32, bool) = ("john".to_string(), 60, false);
    println!("tuple3 tuple: {:?}", tuple3);

    let tuple4 = ("kratos", 92, true, nums);
    println!("tuple4 tuple: {:?}", tuple4);

// Slices
    let num_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("number slices: {:?}", num_slices);

    let animal_slices: &[&str] = &["lion", "elephant", "crocodile"];
    println!("animal slices: {:?}", animal_slices);

    let book_slices: &[String] = &["GOT".to_string(), "Harry Potter".to_string(), "LOTR".to_string()];
    println!("book slices: {:?}", book_slices);
    
    let food_slices: &[&String] = &[&"breakfast".to_string(), &"lunch".to_string(), &"dinner".to_string()];
    println!("food slices: {:?}", food_slices);

// Strings vs String Slices (&str)
// Strings [growable, mutable, owned string type, stored on heap, slow] 
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

// String Slice (&str)
// [immutable, stored on stack, fast]
    let string: String = String::from("hello rust");
    let slice1: &str = &string;
    println!("Slice value: {}", slice1);

    let slice2: &str = &string[6..];
    println!("Slice value: {}", slice2);
    
}