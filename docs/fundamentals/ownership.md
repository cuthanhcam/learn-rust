# Understanding Ownership in Rust

This chapter introduces the core memory model behind Rust: ownership, borrowing, and slices.

These ideas are the reason Rust can guarantee memory safety without a garbage collector, while still giving you control over performance and allocation behavior.

The chapter is split into focused notes so you can study each concept in sequence and revisit the parts that feel unfamiliar.

---

## Chapter Goals

By the end of this chapter, you should be able to explain:

- what ownership means in Rust
- why values move instead of being copied by default
- when Rust copies data automatically and when it does not
- how function calls transfer or preserve ownership
- how borrowing lets code use data without taking it away
- why slices are useful when you only need part of a string or collection

---

## Table of Contents

- [What Is Ownership?](../../fundamentals/ownership/docs/what-is-ownership.md)
    - Ownership in one sentence
    - Stack and heap memory
    - The ownership rules
    - Scope and dropping values
    - `String` and heap allocation
    - Moves versus copies
    - Ownership in functions
    - Returning ownership
- [References and Borrowing](../../fundamentals/ownership/docs/references-and-borrowing.md)
    - Immutable references
    - Mutable references
    - Borrowing rules
    - Passing references to functions
    - Dangling references
- [The Slice Type](../../fundamentals/ownership/docs/slice-type.md)
    - String slices
    - Array and vector slices
    - Range syntax for slices
    - Slices as borrowed views

---

## Detailed Notes

### What Is Ownership?

This is the core chapter note for section 4.1 of the Rust book.

It explains:

- ownership as a single-owner model
- stack versus heap memory
- why `String` behaves differently from fixed-size stack values
- moves, `Copy`, and function ownership transfer

→ [Read the full note](../../fundamentals/ownership/docs/what-is-ownership.md)

---

### References and Borrowing

This note expands the ownership model into borrowing.

It covers:

- shared references with `&T`
- mutable references with `&mut T`
- how Rust prevents conflicting access
- why borrowing is safer than manual pointer management

→ [Read the full note](../../fundamentals/ownership/docs/references-and-borrowing.md)

---

### The Slice Type

This note explains how Rust represents part of a string or collection without copying the entire value.

It covers:

- `&str` as a string slice
- `&[T]` as a slice of array-like data
- range syntax such as `..`, `..end`, and `start..`
- how slices fit into the borrowing model

→ [Read the full note](../../fundamentals/ownership/docs/slice-type.md)

---

## Suggested Reading Order

1. Start with [What Is Ownership?](../../fundamentals/ownership/docs/what-is-ownership.md)
2. Continue with [References and Borrowing](../../fundamentals/ownership/docs/references-and-borrowing.md)
3. Finish with [The Slice Type](../../fundamentals/ownership/docs/slice-type.md)

That order follows the learning path in the Rust book and keeps the concepts building on each other naturally.

---

## Source Code

The current crate entry point is [fundamentals/ownership/src/main.rs](../../fundamentals/ownership/src/main.rs).

It still contains the starter binary, so you can add short example snippets there as you work through the chapter and turn the notes into runnable demonstrations.

---

## Why This Chapter Matters

Ownership is not just one topic in Rust. It is the model that explains:

- memory cleanup
- safe access to heap data
- why borrowing exists
- why Rust can catch many bugs before runtime

If the chapter feels abstract at first, that is normal. The key is to connect each rule back to a concrete question: who owns the data, who can use it, and when does it get cleaned up?
