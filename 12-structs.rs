// Structs

fn main() {
    // tuple
    let rect = (200, 500);

    // struct
    struct Book{
        title: String,
        author: String,
        pages: u32, 
        available: bool,
    }

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("someone"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 4,
    };

    user1.email = String::from("another@gmail.com");
    println!("user1: \n {:?}", user1);

    // return struct from a fn
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email, 
            username,
            sign_in_count: 1,
        }
    }

    println!("using build user:\n {:?}", build_user(String::from("hehe@gmail.com"), String::from("hehe")));

    // create instances from other instances
    let user2 = User{
        email: String::from("another2@gmail.com"),
        ..user1
    };
    println!("user2: \n {:?}", user2);

    // tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}