# Rust Guessing Game Tutorial

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

## Key Concepts Learned

This step introduces these foundational Rust concepts:

- **Mutability** (`let`, `mut`) — Variables are immutable by default; `mut` makes them mutable
- **References** (`&`, `&mut`) — Borrowing data without taking ownership
- **Input/Output** (`std::io`, `stdin()`) — Standard library I/O operations
- **Result Type** (`Ok`, `Err`, `expect()`) — Error handling pattern
- **Printing** (`println!`, `{}` placeholders) — Output formatting

---

## Step 2: Generating a Random Number

To make the game interesting, we need to generate a different secret number each time. This introduces the concept of **external crates** — libraries of code written by other developers.

### Understanding Crates and Dependencies

A **crate** is a package of Rust code. There are two types:

- **Binary crates** — executable programs (like our guessing game)
- **Library crates** — code intended to be used in other programs (like the `rand` crate)

The `rand` crate provides random number generation functionality that isn't included in Rust's standard library.

### Adding the `rand` Crate

Edit your `Cargo.toml` file and add `rand` to the `[dependencies]` section:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

The version `"0.8.5"` means: "use version 0.8.5 or any compatible patch version" (i.e., 0.8.5 to 0.8.99, but not 0.9.0 or higher).

When you run `cargo build`, Cargo will:

1. Download the `rand` crate from crates.io
2. Download all dependencies that `rand` needs
3. Compile everything
4. Link it to your project

### Using the `Rng` Trait

```rust
use rand::Rng;
```

This imports the `Rng` trait from the `rand` crate. A **trait** is a collection of methods that a type must implement. The `Rng` trait defines methods for random number generation.

**Why import the trait?** This is a crucial Rust pattern: the type `rand::rngs::ThreadRng` exists, but Rust knows **nothing about what methods it has** until you import the trait that implements those methods. Without importing `Rng`, you could reference the type but couldn't actually call `gen_range()`. Trait methods are only available when the trait is in scope. This design gives Rust fine-grained control over what's accessible.

### Generating the Secret Number

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

This line generates a random integer between 1 and 100.

**Breaking it down:**

- `rand::thread_rng()` — returns a random number generator specific to the current thread
- `.gen_range(1..=100)` — generates a random number in the inclusive range 1 to 100
    - `1..=100` — the `..=` syntax means "inclusive on both ends"
    - `1..100` (without the `=`) would mean "1 to 99" (exclusive on the right)

### How `thread_rng()` Works

The `thread_rng()` function returns a random number generator that is:

- **Thread-local** — each thread gets its own generator
- **Seeded by the OS** — uses system entropy for randomness
- **Efficient** — fast and suitable for most use cases

---

## Step 3: Comparing Guesses

Now we compare the user's guess with the secret number. This introduces **enums** and **pattern matching**.

### The `Ordering` Enum

```rust
use std::cmp::Ordering;
```

The `Ordering` enum from the standard library represents the result of a comparison. It has three variants:

```rust
Ordering::Less      // first value is less than second
Ordering::Greater   // first value is greater than second
Ordering::Equal     // values are equal
```

An enum is a type that can be one of several variants. Think of it as a value that can be in one of multiple "states."

### The `cmp()` Method

```rust
guess.cmp(&secret_number)
```

The `cmp()` (compare) method is available on any type that can be compared. It returns an `Ordering` enum:

- Returns `Ordering::Less` if `guess < secret_number`
- Returns `Ordering::Greater` if `guess > secret_number`
- Returns `Ordering::Equal` if `guess == secret_number`

**Note on the reference (`&`):** Notice we pass `&secret_number` (a reference) rather than the value itself. The `cmp()` method signature is `fn cmp(&self, other: &Self)`, so it takes references to avoid copying. Even though `u32` is cheap to copy, Rust's API design is consistent: methods take references to avoid unnecessary moves. This is part of Rust's philosophy of explicit ownership.

### Pattern Matching with `match`

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

The `match` expression compares a value against a series of patterns and executes code based on which pattern matches.

**Structure of a match expression:**

```rust
match value {
    pattern1 => code_to_run,
    pattern2 => code_to_run,
    pattern3 => code_to_run,
}
```

**How it works:**

1. Rust evaluates `guess.cmp(&secret_number)` and gets an `Ordering` value
2. Rust checks this value against each pattern in order
3. When a pattern matches, the corresponding code executes
4. The match expression then completes

**Important:** Rust forces you to handle all possible cases. If you forget a case, the code won't compile.

### Understanding Type Coercion

There's a subtle type mismatch here: `guess` is a `String` but `secret_number` is a number. The `cmp()` method can't compare different types. We'll fix this in the next section with type conversion.

---

## Step 4: Type Conversion — Why Types Must Match

This step introduces **shadowing** and the process of converting a `String` to a number. Unlike some languages, Rust does **not** implicitly coerce types. If types don't match, you must explicitly convert them.

### The Type Mismatch Problem

Initially, after reading user input, `guess` is a `String` containing text like `"42"`. But `secret_number` is a number type (likely `i32`). We can't compare a string to a number directly.

**Compiler error:**

```
error[E0308]: mismatched types
expected reference `&String`, found reference `&{integer}`
```

### Shadowing Variables

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

This line creates a new variable also named `guess`, but with type `u32`. This is called **shadowing** — the new variable "hides" the old one.

**Why shadow instead of creating a new variable?**

- Cleaner code — no need for names like `guess_str` and `guess_num`
- Semantically clearer — you're converting the same value, not creating something different
- Common pattern in Rust when converting between types

**Key distinction:** Shadowing allows changing the **type** of a variable (from `String` to `u32`), while `mut` only allows changing the **value** within the same type. That's why `let mut guess = String::new(); guess = 42;` would fail—you're trying to assign a number to a string variable. Shadowing with `let` gives you more flexibility.

### The `trim()` Method

```rust
guess.trim()
```

The `trim()` method removes whitespace from both ends of a string. This is important because `read_line()` includes the newline character that the user presses after entering their guess.

**Example:**

```rust
let input = "42\n";
let trimmed = input.trim();  // "42"
```

**Important detail:** Rust's `str::parse::<u32>()` actually ignores leading and trailing whitespace already, so technically `"42\n".parse()` would work. However, calling `trim()` is idiomatic and makes the intent explicit—you're preparing a string for parsing by removing unwanted whitespace.

### The `parse()` Method

```rust
guess.trim().parse()
```

The `parse()` method converts a string to another type. It returns a `Result`:

- `Ok(value)` — conversion succeeded, contains the parsed number
- `Err(error)` — conversion failed

We need to tell Rust what type to parse into. In this case, `u32` (unsigned 32-bit integer).

**Why `u32`?**

- `u` means unsigned (can't be negative)
- `32` means 32 bits (can hold values 0 to 4,294,967,295)
- Good default for small positive numbers like guesses between 1-100

### Complete Type Conversion

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

**Step by step:**

1. `guess.trim()` — removes newline: `"42\n"` → `"42"`
2. `.parse()` — attempts conversion to `u32`: returns `Result`
3. `.expect("Please type a number!")` — if `Ok`, returns the `u32` value; if `Err`, crashes with message
4. `let guess: u32 = ...` — bind the result to a new variable named `guess` with type `u32`

Now `guess` is a proper number that can be compared with `secret_number`.

---

## Step 5: Creating a Game Loop

To allow multiple guesses, we wrap the game logic in a loop.

### The `loop` Keyword

```rust
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // ... rest of game logic
}
```

The `loop` keyword creates an **infinite loop** that runs forever until explicitly broken.

### Breaking from the Loop

When the player guesses correctly, we exit the loop using the `break` statement:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;  // Exit the loop
    }
}
```

The `break` statement immediately exits the loop, and execution continues after the loop (which ends the program since the loop is the last thing in `main`).

---

## Step 6: Handling Invalid Input

When a user enters non-numeric input, the `parse()` method returns `Err`. Originally, `.expect()` would crash the program. Instead, we gracefully handle errors using pattern matching.

### The Problem with Panic

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

If the user types `"foo"`, `parse()` returns an error, and `.expect()` crashes the program. This isn't ideal for a game — we want to ask them to try again.

### Using `match` for Error Handling

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

This uses `match` to handle both success and failure:

- `Ok(num)` — if parsing succeeded, get the number
- `Err(_) => continue` — if parsing failed, go to the next loop iteration

### The Underscore Pattern

The `_` (underscore) is a **catch-all pattern** that matches any value. Here, we don't care about the error details, just that an error occurred. `_` says "match any error without binding it to a variable."

### The `continue` Statement

```rust
Err(_) => continue,
```

The `continue` statement:

- Skips the rest of the current loop iteration
- Jumps directly to the next iteration
- In this case, skips the comparison and asks for another guess

---

---

## Final Implementation

```rust
use std::cmp::Ordering;
use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

## What This Project Teaches

This small project introduces:

- Basic I/O handling
- Working with external crates
- Error handling (`Result`)
- Pattern matching (`match`)
- Looping and control flow
- Type conversion and shadowing

---

## Common Pitfalls

### Comparing String with Number

```rust
guess.cmp(&secret_number) // ERROR
```

✔ Fix:

```rust
let guess: u32 = guess.trim().parse().expect("...");
```

---

### Ignoring `Result`

Rust warns if you don’t handle errors:

```rust
io::stdin().read_line(&mut guess);
```

Fix:

```rust
.expect("Failed to read line");
```

---

## Personal Notes

- Rust forces you to handle errors early → safer code
- `match` is more powerful than typical `switch`
- Shadowing is cleaner than creating new variables
- Cargo makes dependency management extremely simple

---

## Mental Model of the Guessing Game

Understanding the conceptual flow is important:

```
┌─────────────────────────────────────────────────┐
│  Start Game                                     │
│  - Print welcome message                        │
│  - Generate secret number (1-100)               │
└────────────┬────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────┐
│  Enter Loop (Infinite)                          │
└────────────┬────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────┐
│  Get User Input                                 │
│  - Prompt for guess                             │
│  - Read from stdin                              │
│  - Store as String                              │
└────────────┬────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────┐
│  Parse & Validate                               │
│  - Trim whitespace                              │
│  - Parse String to u32                          │
└────────────┬────────┬───────────────────────────┘
             │        │
             │        ▼
             │    ❌ Invalid (Err)
             │    └─→ continue (loop restart)
             │
             ▼
         ✅ Valid (Ok)
┌─────────────────────────────────────────────────┐
│  Compare with Secret                            │
│  - Less? Print "Too small!"                     │
│  - Greater? Print "Too big!"                    │
│  - Equal? Print "You win!" → BREAK              │
└────────────┬────────────────────────────────────┘
             │
             ├─→ (if Equal) Exit Loop → End Program
             │
             └─→ (if not Equal) Return to Loop
```

---

## Common Pitfalls and How to Fix Them

### Pitfall 1: Forgetting to Add `mut`

**Problem:**

```rust
let guess = String::new();  // ERROR: can't mutate immutable binding
io::stdin().read_line(&guess).expect("...");
```

**Solution:**

```rust
let mut guess = String::new();  // Correct
io::stdin().read_line(&mut guess).expect("...");
```

---

### Pitfall 2: Type Mismatch in Comparison

**Problem:**

```rust
let guess = String::new();
let secret_number = 42;  // number type

guess.cmp(&secret_number);  // ERROR: can't compare String and integer
```

**Reason:** `guess` is a `String`, but `secret_number` is a number. You can't compare different types.

**Solution:** Convert the string to a number:

```rust
let guess: u32 = guess.trim().parse().expect("...");
let secret_number = 42;

guess.cmp(&secret_number);  // Now both are u32
```

---

### Pitfall 3: Forgetting to Handle the `Result`

**Problem:**

```rust
io::stdin().read_line(&mut guess);  // Compiler warning
```

**Reason:** `read_line()` returns a `Result`, which should be handled.

**Solution:** Use `.expect()` or match:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read");
```

---

### Pitfall 4: Not Trimming Newlines

**Problem:**

```rust
let guess: u32 = guess.parse().expect("...");  // ERROR if guess contains "\n"
```

**Reason:** The input includes the newline character that the user pressed.

**Solution:** Call `.trim()` first:

```rust
let guess: u32 = guess.trim().parse().expect("...");
```

---

### Pitfall 5: Panicking on Invalid Input

**Problem:**

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

If the user types non-numeric input, the program crashes.

**Solution:** Use `match` to handle the error gracefully:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

---

### Pitfall 6: Infinite Loop Without Exit Condition

**Problem:**

```rust
loop {
    // Game logic without a break statement
    // Loop runs forever!
}
```

**Solution:** Add a `break` when the condition is met:

```rust
loop {
    // Game logic
    match guess.cmp(&secret_number) {
        // ...
        Ordering::Equal => {
            println!("You win!");
            break;  // Exit the loop
        }
    }
}
```

---

## Key Takeaways

### 1. Rust Enforces Safety

Rust's strict type system and error handling force you to think about edge cases from the start. This leads to more robust code.

### 2. Variables Are Immutable by Default

The `mut` keyword makes it explicit when a variable can change. This is a design choice that prevents bugs from accidental modifications.

### 3. Pattern Matching Is Powerful

The `match` expression is more flexible than traditional `switch` statements. It forces you to handle all cases and allows complex patterns.

### 4. Traits Enable Code Reuse

By importing the `Rng` trait, you gain access to random number methods. Traits are how Rust enables polymorphism and code sharing.

### 5. External Crates Extend Functionality

Cargo makes it trivial to use community libraries. The `rand` crate demonstrates how the ecosystem enables rapid development.

### 6. Error Handling Is First-Class

The `Result` type makes error handling explicit. You can't accidentally ignore errors in Rust.

### 7. Shadowing Is Idiomatic

Creating a new variable with the same name during type conversion is a common, clean pattern in Rust.

---

## Running and Testing the Game

To build and run:

```bash
$ cargo build
$ cargo run
```

**Sample Game Session:**

```
Guess the number!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
25
You guessed: 25
Too small!
Please input your guess.
37
You guessed: 37
Too big!
Please input your guess.
30
You guessed: 30
Too small!
Please input your guess.
33
You guessed: 33
Too small!
Please input your guess.
35
You guessed: 35
You win!
```

---

## Next Steps

After mastering this project, explore:

1. **Improving the Game**
    - Add a guess counter
    - Set difficulty levels
    - Track high scores

2. **Related Concepts**
    - Functions (custom reusable code)
    - Structs (grouping related data)
    - Enums and Traits (more pattern matching)
    - Collections (vectors, hash maps)

3. **Advanced Error Handling**
    - Using `Result` without `.expect()`
    - Custom error types
    - The `?` operator for error propagation

---

## Reference

- [Official Rust Book — Chapter 2: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [std::io Module Documentation](https://doc.rust-lang.org/std/io/)
- [rand Crate Documentation](https://docs.rs/rand/)
- [Pattern Matching in Rust](https://doc.rust-lang.org/book/ch06-02-match.html)
