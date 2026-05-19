# Syntax Basics

This documentation covers the basics of Rust syntax and foundational concepts learned in this section.

The goal is to understand simple syntax elements in Rust and how they translate into working programs.

## Learning Goals

- explain immutable vs mutable bindings
- identify scalar and compound data types
- write and call typed functions
- use expression-based control flow with confidence
- read beginner Rust code and predict behavior

---

# Table of Contents

- [Syntax Basics](#syntax-basics)
- [Table of Contents](#table-of-contents)
    - [Variables](#variables)
        - [Example:](#example)
    - [Data Types](#data-types)
        - [Scalar Types](#scalar-types)
        - [Compound Types](#compound-types)
        - [Example:](#example-1)
    - [Functions](#functions)
        - [Example:](#example-2)
    - [Control Flow](#control-flow)
        - [Example of `if`:](#example-of-if)
        - [Example of `match`:](#example-of-match)
    - [Mental Model of Rust Syntax](#mental-model-of-rust-syntax)

---

## Variables

Variables in Rust are immutable by default, but they can be made mutable using the `mut` keyword.

### Example:

```rust
let x = 5;       // immutable
let mut y = 10;  // mutable
```

- `let` creates a new variable
- `mut` allows the variable to be reassigned

---

## Data Types

Rust has strong, static typing. The key data types include:

### Scalar Types

- `i32`: Integer
- `f64`: Floating-point number
- `bool`: Boolean value
- `char`: Single character

### Compound Types

- `Tuple`: Groups multiple values of different types
- `Array`: A fixed-size collection of values of the same type

### Example:

```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'R';

let tuple: (i32, f64, char) = (10, 2.5, 'A');
let (a, b, c) = tuple;
```

---

## Functions

In Rust, functions are declared using `fn` and can return values either implicitly (as the last expression) or explicitly using `return`.

### Example:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- The `add` function takes two `i32` arguments and returns their sum.

---

## Control Flow

Rust uses common control flow constructs such as `if`, `while`, `loop`, and `match`.

### Example of `if`:

```rust
if x > 5 {
    println!("Greater than 5");
}
```

### Example of `match`:

```rust
match x {
    1 => println!("One"),
    _ => println!("Other"),
}
```

---

## Mental Model of Rust Syntax

Rust is designed with safety and performance in mind. Its syntax promotes:

- **Immutability by default**
- **Explicit error handling (using `Result` and `Option`)**
- **Memory safety through ownership, borrowing, and lifetimes**

Rust emphasizes:

- High performance (comparable to C/C++)
- Safety (compile-time guarantees instead of runtime checks)

---

## Common Pitfalls

- forgetting `mut` when reassigning a variable
- expecting `if` to accept non-boolean conditions
- mixing statement and expression forms by accident
- assuming tuple/array indexing never fails at runtime
