# Data Types in Rust

Rust is a statically typed language, so the compiler must know the type of every variable at compile time.

That gives Rust strong safety guarantees, but it also means you need to be deliberate about how you represent data.

The demo code for this lesson lives in [data_types.rs](../src/data_types.rs).

---

## Table of Contents

1. Scalar Types
2. Type Annotations
3. Compound Types
4. Summary
5. Reference

---

## Scalar Types

Scalar types represent a single value. Rust's four primary scalar types are:

- integers
- floating-point numbers
- Booleans
- characters

### Integers

Integer types are signed or unsigned and come in different widths.

Common defaults and rules:

- integer literals default to `i32` when the compiler can infer the type
- `usize` and `isize` are commonly used for indexing and collection sizes
- `u8` can store values from 0 to 255
- integer overflow can panic in debug builds or wrap in release builds

### Floating-point numbers

Rust has two floating-point types:

- `f32`
- `f64`

The default is `f64` because it usually gives the best balance of precision and speed.

### Booleans

Booleans have only two values:

- `true`
- `false`

They are commonly used in `if` expressions and other control-flow conditions.

### Characters

Rust's `char` type stores a Unicode scalar value.

That means it can represent much more than ASCII, including:

- accented letters
- symbols
- emoji

### Numeric operations

Rust supports the usual numeric operators:

- addition
- subtraction
- multiplication
- division
- remainder

Integer division truncates toward zero.

---

## Type Annotations

Rust can often infer types, but sometimes you must make them explicit.

This becomes necessary when multiple types are valid, such as when parsing text:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Type annotations are also required for function parameters and return types.

### Why annotations matter

When the compiler knows a type early, it can:

- produce better error messages
- catch mismatches at compile time
- avoid guesswork in ambiguous code

---

## Compound Types

Compound types group multiple values into one type. Rust's two primitive compound types are tuples and arrays.

### Tuples

Tuples can hold values of different types.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

You can access tuple values in two ways:

- destructuring with `let (x, y, z) = tup;`
- index access with `tup.0`, `tup.1`, and so on

Tuples have a fixed length, so once a tuple is created, its size cannot change.

### Arrays

Arrays store values of the same type and also have a fixed length.

```rust
let a = [1, 2, 3, 4, 5];
let repeated = [3; 5];
```

Arrays are useful when:

- the number of values is known in advance
- the values should live on the stack
- you want a fixed-size collection

Array access uses indexing, such as `a[0]`.

If the index is out of bounds, Rust checks it at runtime and panics.

---

## Summary

1. Rust is statically typed
   -> types must be known at compile time

2. Scalar types hold one value
   -> integers, floats, Booleans, and chars

3. Compound types group values together
   -> tuples and arrays

4. Type annotations remove ambiguity
   -> especially for parsing and function signatures

5. Arrays are fixed-size and same-type
   -> tuples are fixed-size but can mix types

---

## Reference

- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example: Data Types](https://doc.rust-lang.org/rust-by-example/primitives.html)
