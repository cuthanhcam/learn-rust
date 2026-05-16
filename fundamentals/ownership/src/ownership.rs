//! Ownership: stack and heap values, move, copie and function transfer.
//!
//! - ownership as a single-owner model
//! - stack values versus heap-allocated values
//! - `Copy` types versus moved values
//! - ownership in function calls and returns

pub fn run() {
    println!("\n-- Ownership --");

    ownership_rules();
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
