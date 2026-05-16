//! Slice types: borrowed views into strings and collections.
//!
//! - string slices (`&str`)
//! - array and vector slices (`&[T]`)
//! - ranges for selecting subparts of data
//! - slices as borrowed, non-owning views

pub fn run() {
    println!("\n-- Slice Type --");

    string_slices();
    collection_slices();
    first_word_example();
}

/// Show that a string slice borrows part of a `String`.
fn string_slices() {
    println!("\n[1] String Slices");

    let text = String::from("hello rust world");
    let hello = &text[0..5];
    let rust = &text[6..10];

    println!("text = {text}");
    println!("hello = {hello}");
    println!("rust = {rust}");
    println!("Slices let you work with part of a string without copying the whole value.");
}

/// Show slices over arrays and vectors.
fn collection_slices() {
    println!("\n[2] Array and Vector Slices");

    let numbers = [10, 20, 30, 40, 50, 60];
    let middle = &numbers[1..5];
    let all_numbers = &numbers[..];

    println!("numbers = {numbers:?}");
    println!("middle = {middle:?}");
    println!("all_numbers = {all_numbers:?}");
    println!("Slices borrow a contiguous range of data from the original collection.");
}

/// Demonstrate a common slice-based parsing pattern.
fn first_word_example() {
    println!("\n[3] First Word Pattern");

    let sentence = String::from("rust makes ownership explicit");
    let first = first_word(&sentence);

    println!("sentence = {sentence}");
    println!("first word slice = {first}");
    println!("This is the classic pattern used when you want part of a string without owning it.");
}

/// Return a slice containing the first word in the input string.
fn first_word(text: &str) -> &str {
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }

    text
}
