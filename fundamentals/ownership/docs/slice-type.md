# The Slice Type in Rust

A slice is a view into a contiguous sequence of elements.

Slices let you work with part of a collection or part of a string without taking ownership of the whole value.

In Rust, slices are usually represented as references, such as `&str` for string slices and `&[T]` for slices of arrays or vectors.

---

## Table of Contents

1. Why Slices Exist
2. String Slices
3. Array and Vector Slices
4. Slice Lengths and Ranges
5. Slices and Borrowing
6. Summary

---

## Why Slices Exist

Many programs need to work with a portion of a larger value.

Examples:

- the first word in a sentence
- a range of numbers in a list
- a segment of bytes from a buffer
- a substring inside a larger string

Instead of copying that data, Rust lets you borrow just the part you need.

---

## String Slices

A string slice is a reference to some UTF-8 text.

```rust
let text = String::from("hello world");
let part = &text[0..5];
```

`part` does not own the text. It borrows a view into `text`.

### Why this matters

- no extra allocation
- no copying the entire string
- safer APIs for parsing and searching

---

## Array and Vector Slices

Slices also work with fixed-size arrays and vectors.

```rust
let numbers = [1, 2, 3, 4, 5];
let middle = &numbers[1..4];
```

The slice refers to the range of items in the original collection.

That means code can inspect or pass around a subset of the data without cloning the whole thing.

---

## Slice Lengths and Ranges

Slice syntax uses ranges to choose the part you want.

```rust
let a = &text[0..5];
let b = &text[..5];
let c = &text[6..];
let d = &text[..];
```

### Common forms

- `start..end` includes `start` and excludes `end`
- `..end` starts from the beginning
- `start..` goes to the end
- `..` borrows the entire sequence

---

## Slices and Borrowing

Slices are a practical example of borrowing in action.

They let you:

- read part of data without owning it
- avoid copying large collections
- write functions that are flexible about input size

Because slices borrow data, they are subject to the same validity rules as other references.

---

## Summary

Slices are borrowed views into part of a value.

The most important forms are:

- `&str` for text slices
- `&[T]` for slices of arrays and vectors

Slices are useful whenever you want part of a collection without taking ownership of the whole thing.
