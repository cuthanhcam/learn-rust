# Comments in Rust

Comments are notes for humans. The compiler ignores them, so they do not affect runtime behavior.

The demo code for this lesson lives in [comments.rs](../src/comments.rs).

---

## Table of Contents

1. Line Comments
2. Trailing Comments
3. Multi-line Comments
4. Documentation Comments
5. Summary
6. Reference

---

## Line Comments

Rust uses `//` for line comments.

```rust
// This is a comment.
let lucky_number = 7;
```

You can use line comments to explain:

- why a piece of code exists
- tricky implementation details
- assumptions that are not obvious from the code alone

Avoid using comments to restate what the code already says clearly.

---

## Trailing Comments

You can place a comment after code on the same line:

```rust
let speed = 10; // comment after code
```

This is valid, but separate lines are often easier to scan and maintain.

Use trailing comments sparingly when the note is short and directly related to the line.

---

## Multi-line Comments

Rust also supports block comments.

```rust
/*
This comment spans multiple lines.
It can be useful for longer explanations or temporarily disabling code.
*/
```

Block comments are still ignored by the compiler.

---

## Documentation Comments

Rust also has documentation comments, written with `///`.

These comments are used to generate API documentation for functions, types, modules, and other items.

```rust
/// Documentation comments describe the item below them.
fn example() {}
```

The Rust Book introduces documentation comments here, but explains them in more depth later.

---

## Summary

1. Comments are ignored by the compiler
   -> they exist for readers, not for execution

2. `//` is the standard line comment style
   -> use it for short explanations

3. `/* */` supports block comments
   -> useful for longer notes or temporary code disabling

4. `///` is for documentation comments
   -> use it when you want generated docs

5. Good comments explain intent
   -> they should add information the code itself does not make obvious

---

## Reference

- [Rust Book: Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
