//! Ownership: stack and heap values, move, copie and function transfer.
//!
//! - ownership as a single-owner model
//! - stack values versus heap-allocated values
//! - `Copy` types versus moved values
//! - ownership in function calls and returns

pub fn run() {
    println!("\n-- Ownership --");

    ownership_rules();
    stack_vs_heap();
    copy_vs_move();
    ownership_and_functions();
    ownership_and_return_values();
}

/// Show the three core ownership rules with a scoped `String`.
fn ownership_rules() {
    println!("\n[1] Ownership Rules");
    {
        let message = String::from("ownership gives each value one onwer");
        println!("message = {message}");
        println!("The owner of 'message' is the variable that currently holds it.");
        println!("When the block ends, Rust automatically drops the `String`.");
    }

    println!("After the scope ends, the value has already been cleaned up.");
}

/// Compare stack-allocated values with heap-allocated data.
fn stack_vs_heap() {
    println!("\n[2] Stack vs Heap");

    let integer = 42;
    let floating_point = 3.5;
    let boolean = true;
    let text = String::from("stored on the heap");

    println!("integer = {integer}");
    println!("floating_point = {floating_point}");
    println!("boolean = {boolean}");
    println!("text = {text}");
    println!("Integers, floats, and booleans are small stack values with known size.");
    println!("String owns heap memory so it can grow and change at runtime.");
}

/// Show the difference between implicit copies and ownership moves.
fn copy_vs_move() {
    println!("\n[3] Copy vs Move");

    let x = 7;
    let y = x;

    println!("x = {x}, y = {y}");
    println!("`i32` implements `Copy`, so assigning it duplicates the value.");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {s2}");
    println!("`String` does not implement `Copy`, so ownership moves from `s1` to `s2`.");
    println!("After the move, `s1` is no longer valid and cannot be used.");
}

/// Show how passing values into functions changes ownership.
fn ownership_and_functions() {
    println!("\n[4] Ownership and Functions");

    let score = 100;
    let score_after_call = takes_copy(score);

    println!("score = {score}");
    println!("score_after_call = {score_after_call}");
    println!("Copy types stay usable after a function call.");

    let message = String::from("Rust passes heap data by move unless you borrow it");
    let message_length = takes_and_measures(message.as_str());

    println!("message_length = {message_length}");
    println!("The original `String` was moved into the function and returned as a length.");
}

/// Show returning ownership back to the caller.
fn ownership_and_return_values() {
    println!("\n[5] Returning Ownership");

    let original = String::from("keep me");
    let (returned, length) = take_and_return(original);

    println!("returned = {returned}");
    println!("length = {length}");
    println!("The function consumed the `String`, inspected it, and gave ownership back.");
}

/// Copying an integer keeps the original available.
fn takes_copy(value: i32) -> i32 {
    println!("takes_copy received {value}");
    value + 1
}

/// Borrowing a string slice keeps ownership with the caller while still allowing inspection.
fn takes_and_measures(text: &str) -> usize {
    println!("takes_and_measures received: {text}");
    text.len()
}

/// Consume a string and return both the string and its length.
fn take_and_return(text: String) -> (String, usize) {
    let length = text.len();
    (text, length)
}
