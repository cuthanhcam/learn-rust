//! Borrowing: shared references, mutable references, and function parameters.
//!
//! - immutable references for read-only access
//! - mutable references for controlled modification
//! - the no-conflict borrow rule
//! - references as function parameters

pub fn run() {
    println!("\n-- Borrowing --");

    immutable_borrowing();
    mutable_borrowing();
    borrow_rules();
    function_borrowing();
}

/// Show how shared references let you read without taking ownership.
fn immutable_borrowing() {
    println!("\n[1] Immutable References");

    let message = String::from("borrowed data can be read safely");
    let first_view = &message;
    let second_view = &message;

    println!("first_view = {first_view}");
    println!("second_view = {second_view}");
    println!("message is still usable because both references are read-only.");
}

/// Show how mutable references allow controlled in-place updates.
fn mutable_borrowing() {
    println!("\n[2] Mutable References");

    let mut message = String::from("hello");
    append_world(&mut message);

    println!("message = {message}");
    println!("The value was modified without copying the entire string.");
}

/// Demonstrate the borrow rules in a practical way.
fn borrow_rules() {
    println!("\n[3] Borrow Rules");

    let mut numbers = vec![1, 2, 3];

    {
        let read_only = &numbers;
        println!("read_only = {read_only:?}");
    }

    {
        let mutable_view = &mut numbers;
        mutable_view.push(4);
        mutable_view.push(5);
    }

    println!("numbers = {numbers:?}");
    println!("Rust allows either many readers or one writer at a time.");
}

/// Show references as function parameters.
fn function_borrowing() {
    println!("\n[4] Borrowing in Functions");

    let text = String::from("function parameters often borrow");
    let length = calculate_length(text.as_str());

    println!("text = {text}");
    println!("length = {length}");
    println!("The function read the string without taking ownership.");
}

/// Mutably borrow a string and extend it in place.
fn append_world(message: &mut String) {
    message.push_str(", world");
}

/// Read a string by reference and return its length.
fn calculate_length(text: &str) -> usize {
    text.len()
}
