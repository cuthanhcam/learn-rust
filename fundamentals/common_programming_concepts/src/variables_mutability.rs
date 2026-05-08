//! This module demonstrates how Rust handles.
//!
//! - immutable variables (default)
//! - mutable variables (`mut`)
//! - constants
//! - shadowing
//! - scope-based shadowing
//! - type transformation via shadowing
//! - `mut` vs shadowing differences

// Entry point
pub fn run() {
    println!("\n-- Variables and Mutability --");

    immutable_variables();
}

/// 1. Immutable Variables
fn immutable_variables() {
    println!("\n[1] Immutable Variables");

    let x = 5;
    println!("Initial value of x: {x}");

    // x = 10; // compile-time error: immutable variable

    println!("x is immutable, so it cannot be changed after assignment.");
}
