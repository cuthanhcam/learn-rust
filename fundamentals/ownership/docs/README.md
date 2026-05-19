# Understanding Ownership in Rust

This local index groups the ownership chapter notes in one place.

## Topics

- [What Is Ownership?](./what-is-ownership.md)
- [References and Borrowing](./references-and-borrowing.md)
- [The Slice Type](./slice-type.md)

## Learning Goals

- track ownership flow through assignments and function calls
- decide when to move, copy, or borrow
- understand how slices provide non-owning views

## Common Pitfalls

- using moved values
- conflicting mutable/immutable borrows
- returning references to dropped data

## Suggested Reading Order

1. What Is Ownership?
2. References and Borrowing
3. The Slice Type

## Practice Tasks

1. Refactor one function to take `&str` instead of `String`.
2. Implement a safe first-word slice function.
3. Write examples of move, copy, and borrow side by side.
