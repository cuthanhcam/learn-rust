# Functions in Rust

Functions are named blocks of code that let you reuse behavior and keep programs organized.

Rust uses snake case for function names, and function signatures are type-aware from the start.

The demo code for this lesson lives in [functions.rs](../src/functions.rs).

---

## Table of Contents

1. Defining and Calling Functions
2. Parameters
3. Statements and Expressions
4. Return Values
5. Summary
6. Reference

---

## Defining and Calling Functions

You define a function with `fn`, a name, parentheses, and a body:

```rust
fn another_function() {
	 println!("Another function.");
}
```

You call it by writing its name with parentheses:

```rust
another_function();
```

Rust does not care whether you define the function before or after the call site, as long as it is visible in scope.

### Why this matters

Functions let you:

- split programs into focused pieces
- reuse behavior
- make code easier to read and test

---

## Parameters

Parameters are the named inputs in a function signature.

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
	 println!("The measurement is: {value}{unit_label}");
}
```

Important rules:

- parameter types must be declared
- parameters are part of the function signature
- the value passed at the call site is called an argument

### Multiple parameters

Separate parameters with commas.

This makes function contracts explicit and helps the compiler catch mistakes early.

---

## Statements and Expressions

Rust is an expression-oriented language, so understanding the difference matters.

### Statements

Statements perform an action and do not return a value.

Examples:

- `let` bindings
- function definitions

### Expressions

Expressions evaluate to a value.

Examples:

- arithmetic like `5 + 6`
- function calls
- block expressions

### Block expressions

The value of a block is the value of its last expression, as long as that last expression does not end with a semicolon.

```rust
let y = {
	 let x = 3;
	 x + 1
};
```

If you add a semicolon to the final line, the block returns `()` instead.

---

## Return Values

Functions can return values by declaring a return type after `->`.

```rust
fn plus_one(x: i32) -> i32 {
	 x + 1
}
```

Key points:

- the last expression is the implicit return value
- `return` can be used for early exits
- the return type must match the function body

### Common patterns

- `five() -> i32` for a constant result
- `plus_one(x: i32) -> i32` for small transformations
- `return` when a branch should exit immediately

---

## Summary

1. Functions package reusable behavior
   -> keep code organized and readable

2. Parameters require types
   -> the signature is part of the contract

3. Statements and expressions are different
   -> expressions produce values, statements do not

4. Function bodies return the last expression by default
   -> omit the semicolon when you want a value

5. `return` is available for early exits
   -> use it when control flow needs to stop immediately

---

## Reference

- [Rust Book: How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
