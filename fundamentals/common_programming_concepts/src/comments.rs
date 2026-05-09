//! Comments: demonstrate line, trailing, block, and documentation comments.
//!
pub fn run() {
    println!("\n-- Comments --");

    line_comments();
    trailing_comments();
    multi_line_comments();
    documentation_comments();
}

/// Show single-line comments with `//` and how to use them.
fn line_comments() {
    println!("\n[1] Line Comments");
    println!("Rust uses // for comments that continue to the end of the line.");

    // This line is a comment and is ignored by the compiler.
    let lucky_number = 7;

    println!("lucky_number = {lucky_number}");
    println!("Use comments to explain why code exists, not to restate obvious syntax.");
}

/// Example of a trailing comment after a statement.
fn trailing_comments() {
    println!("\n[2] Trailing Comments");

    let speed = 10; // comment after code

    println!("speed = {speed}");
    println!("Trailing comments are fine, but separate lines are often easier to read.");
}

/// Demonstrate block comments (`/* ... */`) for longer notes.
fn multi_line_comments() {
    println!("\n[3] Multi-line Comments");

    /*
    Rust also supports block comments.
    They are useful when you need to temporarily comment out a block of code.
    */

    println!("Block comments are still ignored by the compiler.");
}

/// Documentation comments (`///`) are used for generated API docs.
fn documentation_comments() {
    println!("\n[4] Documentation Comments");
    println!("Documentation comments start with /// and describe the item below them.");
    println!("The Rust Book introduces them here but discusses them in more depth later.");
}
