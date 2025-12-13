// References and Borrowing
// Safety and Performance

// References: enable you to borrow values without taking ownership
// Immutable References: allow you to read data without modifying it
// Mutable References: allow you to modify data
// Create Reference by adding '&'

// fn main() {
//     // Immutable Reference (default)
//     // let _x = 5;
//     // let _r = &_x; // immutable reference to _x
//     // *_r += 1; // error: cannot assign to data through an immutable reference
//     // println!("value of _x: {}", _x);
//     // println!("value of _r: {}", _r);

//     // Mutable Reference ('mut' keyword)
//     let mut _x = 5;
//     let _r = &mut _x; // mutable reference to _x
//     *_r += 1; // modify the value through the mutable reference
//     *_r -= 3;

//     println!("value of _x: {}", _x);
//     // println!("value of _r: {}", _r); // error: cannot print mutable reference directly
// }

// One Mutable Reference or Multiple Immutable References
fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 155.5,
    };

    // Immutable reference to check balance
    account.check_balance();

    // Mutable reference to withdraw amount
    account.withdraw(50.0);

    // Immutable reference to check balance
    account.check_balance();

    // Mutable reference to withdraw amount
    account.withdraw(5.0);

    // Immutable reference to check balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f32,
}

impl BankAccount {
    // ensure no simultaneous mutable references to account to update balance
    // and immutable references to read owner
    fn withdraw(&mut self, amount: f32) {
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("{}'s balance is {}", self.owner, self.balance);
    }
}