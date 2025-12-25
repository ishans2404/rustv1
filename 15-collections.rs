// Collections:
// Vectors, UTF-8, Hash Maps


// Vectors
// fn main() {
//     let _v: Vec<i32> = Vec::new();
//     let _vec2: Vec<i32> = vec![1, 2, 3, 5];

//     let mut _nums: Vec<i32> = vec![1, 2, 5];
//     _nums.push(6);
//     _nums.push(10);
//     _nums.push(13);
//     println!("_nums: {:?}", _nums);

//     // direct indexing
//     let third: &i32 = &_nums[3];
//     println!("third element: {third}");

//     let third = _nums.get(3);
//     match third {
//         Some(third) => println!("third element is {third}"),
//         None => println!("no third element"),
//     }

// }


// UTF8
// fn main() {
//     // 1
//     let s = "some text1".to_string();
//     // 2
//     let s = String::from("some text2");
//     // mutable
//     let mut s = String::from("some text3");
//     s.push_str(" more text");
//     println!("s: {}", s);

//     s.push('!');
//     println!("s: {}", s);

//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");

//     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//     println!("s3: {}", s3);

// }


// Hash Maps
fn main() {
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}