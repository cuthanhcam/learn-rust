# Rust Guessing Game

This comprehensive guide walks through building the classic **Guessing Game** from the official Rust Book. It marks the transition from basic syntax into **real program flow, user input, external crates, error handling, and pattern matching**.

This project brings together fundamental Rust concepts and demonstrates how they work together in a practical, interactive program.

---

## Goal of the Project

Build a fully functional CLI game that:

- Generates a random secret number between 1 and 100
- Accepts user input via standard input
- Compares the user's guess against the secret number
- Provides feedback (too small, too big, or correct)
- Repeats the game loop until the user wins
- Gracefully handles invalid input without crashing

---

## Table of Contents

- [Rust Guessing Game Tutorial](#rust-guessing-game-tutorial)
    - [Goal of the Project](#goal-of-the-project)
    - [Table of Contents](#table-of-contents)
    - [Project Setup](#project-setup)
        - [Creating a New Cargo Project](#creating-a-new-cargo-project)
        - [Project Structure](#project-structure)
        - [Understanding Cargo.toml](#understanding-cargotoml)
    - [Step 1: Processing User Input](#step-1-processing-user-input)
        - [Importing the `std::io` Module](#importing-the-stdio-module)
        - [Creating a Mutable String](#creating-a-mutable-string)
        - [Reading User Input](#reading-user-input)
        - [Handling Errors with `Result` and `expect()`](#handling-errors-with-result-and-expect)
        - [Printing User Input](#printing-user-input)
    - [Key Concepts Learned](#key-concepts-learned)
    - [Step 2: Generating a Random Number](#step-2-generating-a-random-number)
        - [Understanding Crates and Dependencies](#understanding-crates-and-dependencies)
        - [Adding the `rand` Crate](#adding-the-rand-crate)
        - [Using the `Rng` Trait](#using-the-rng-trait)
        - [Generating the Secret Number](#generating-the-secret-number)
        - [How `thread_rng()` Works](#how-thread_rng-works)
    - [Step 3: Comparing Guesses](#step-3-comparing-guesses)
        - [The `Ordering` Enum](#the-ordering-enum)
        - [The `cmp()` Method](#the-cmp-method)
        - [Pattern Matching with `match`](#pattern-matching-with-match)
        - [Understanding Type Coercion](#understanding-type-coercion)
    - [Step 4: Type Conversion — Why Types Must Match](#step-4-type-conversion--why-types-must-match)
        - [The Type Mismatch Problem](#the-type-mismatch-problem)
        - [Shadowing Variables](#shadowing-variables)
        - [The `trim()` Method](#the-trim-method)
        - [The `parse()` Method](#the-parse-method)
        - [Complete Type Conversion](#complete-type-conversion)
    - [Step 5: Creating a Game Loop](#step-5-creating-a-game-loop)
        - [The `loop` Keyword](#the-loop-keyword)
        - [Breaking from the Loop](#breaking-from-the-loop)
    - [Step 6: Handling Invalid Input](#step-6-handling-invalid-input)
        - [The Problem with Panic](#the-problem-with-panic)
        - [Using `match` for Error Handling](#using-match-for-error-handling)
        - [The Underscore Pattern](#the-underscore-pattern)
        - [The `continue` Statement](#the-continue-statement)
    - [Final Implementation](#final-implementation)
    - [What This Project Teaches](#what-this-project-teaches)
    - [Common Pitfalls](#common-pitfalls)
        - [Comparing String with Number](#comparing-string-with-number)
        - [Ignoring `Result`](#ignoring-result)
    - [Personal Notes](#personal-notes)
    - [Mental Model of the Guessing Game](#mental-model-of-the-guessing-game)
    - [Common Pitfalls and How to Fix Them](#common-pitfalls-and-how-to-fix-them)
        - [Pitfall 1: Forgetting to Add `mut`](#pitfall-1-forgetting-to-add-mut)
        - [Pitfall 2: Type Mismatch in Comparison](#pitfall-2-type-mismatch-in-comparison)
        - [Pitfall 3: Forgetting to Handle the `Result`](#pitfall-3-forgetting-to-handle-the-result)
        - [Pitfall 4: Not Trimming Newlines](#pitfall-4-not-trimming-newlines)
        - [Pitfall 5: Panicking on Invalid Input](#pitfall-5-panicking-on-invalid-input)
        - [Pitfall 6: Infinite Loop Without Exit Condition](#pitfall-6-infinite-loop-without-exit-condition)
    - [Key Takeaways](#key-takeaways)
        - [1. Rust Enforces Safety](#1-rust-enforces-safety)
        - [2. Variables Are Immutable by Default](#2-variables-are-immutable-by-default)
        - [3. Pattern Matching Is Powerful](#3-pattern-matching-is-powerful)
        - [4. Traits Enable Code Reuse](#4-traits-enable-code-reuse)
        - [5. External Crates Extend Functionality](#5-external-crates-extend-functionality)
        - [6. Error Handling Is First-Class](#6-error-handling-is-first-class)
        - [7. Shadowing Is Idiomatic](#7-shadowing-is-idiomatic)
    - [Running and Testing the Game](#running-and-testing-the-game)
    - [Next Steps](#next-steps)
    - [Reference](#reference)

---

## Project Setup

### Creating a New Cargo Project

To start a new Rust project, use the `cargo new` command:

```bash
$ cargo new guessing_game
$ cd guessing_game
```

The first command creates a new directory called `guessing_game` with the basic project structure. The second command changes into that directory.

### Project Structure

After running `cargo new guessing_game`, your project structure looks like:

```
guessing_game/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── main.rs      # Entry point for the program
```

### Understanding Cargo.toml

The `Cargo.toml` file contains project metadata:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- `[package]` section: Contains project metadata
- `name`: The name of your binary
- `edition`: The Rust edition (2021 is widely used; 2024 is the latest)
- `[dependencies]` section: Lists external crates your project depends on

---

## Step 1: Processing User Input

The first step is to accept user input from the player. This introduces the `std::io` module and the concept of mutable variables and references.

### Importing the `std::io` Module

```rust
use std::io;
```

This line brings the input/output functionality from Rust's standard library into scope. The `std::io` module provides tools for reading from standard input and writing to standard output.

**Why do we need this?** By default, Rust includes only a small set of items (called the _prelude_) in every program. The `io` module is not part of the prelude, so we must explicitly import it.

### Creating a Mutable String

```rust
let mut guess = String::new();
```

This line creates a mutable variable named `guess` that holds an empty `String`.

**Breaking it down:**

- `let` — declares a new variable
- `mut` — makes the variable mutable (Rust variables are immutable by default)
- `guess` — the variable name
- `String::new()` — calls the associated function `new()` on the `String` type
- `::` — indicates that `new` is an associated function (a function attached to a type, not an instance)

**Associated Functions:** The `new()` function is called an "associated function" because it's associated with the `String` type. It creates a new, empty string. Many types in Rust have a `new()` function for this reason.

### Reading User Input

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

This code reads a line of input from the user and stores it in the `guess` variable.

**Breaking it down:**

- `io::stdin()` — returns a handle to the standard input stream
- `.read_line(&mut guess)` — calls the `read_line` method, passing a mutable reference to `guess`
- `&mut guess` — `&` creates a reference (allowing data to be accessed without being copied), `mut` makes that reference mutable so `read_line` can modify the string

**Why a reference?** If we passed `guess` directly instead of `&mut guess`, we'd lose ownership of the variable. References allow the method to modify the data without taking ownership.

**Important:** `read_line` appends the user's input to the string (it doesn't replace existing content). It also includes the newline character (`\n`).

### Handling Errors with `Result` and `expect()`

The `.expect("Failed to read line")` part handles potential errors.

The `read_line()` method returns a `Result` type, which is an enum with two variants:

- `Ok(value)` — the operation succeeded
- `Err(error)` — the operation failed

The `expect()` method:

- If `Result` is `Ok`, returns the value inside
- If `Result` is `Err`, crashes the program and prints the message you provided

**Error awareness:** Rust marks `Result` as `#[must_use]`, so ignoring it triggers a compiler warning. This enforces **error awareness**—Rust won't let you silently ignore errors. You must explicitly handle them with methods like `.expect()` or `match`.

### Printing User Input

```rust
println!("You guessed: {guess}");
```

This prints the user's input. The `{guess}` placeholder is replaced with the value of the `guess` variable.

**Note on formatting:** Rust allows inline variable formatting (`{guess}`) instead of positional arguments (`{}`). This is the modern style (Rust 1.58+) and is more readable than older code you might see using `println!("You guessed: {}", guess);`

---
