// Control Flow

// Loops

fn main() {
    // 'loop' keyword 
    // loop {
    //     println!("infinite loop");
    //     break;
    // }


    // let mut counter = 0;
    // let res1 = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result1 is {}", res1);
    // let res2 = loop {
    //     counter += 1;
    //     if counter == 20 {
    //         break counter - 3;
    //     }
    // };
    // println!("The result2 is {}", res2);


    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count: {}", count);
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining: {}", remaining);
    //         if remaining == 5 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }


    // 'while' keyword
    // let mut number = 3;
    // while number != 0 {
    //     println!("number = {number}");
    //     number -= 1;
    // }


    // 'for' keyword
    let a = [10, 20, 30, 40, 50];
    let b = ["apple", "banana", "cherry"];
    for element in a {
        println!("the value is: {}", element);
    }
    for fruit in b {
        println!("the fruit is: {}", fruit);
    }
}