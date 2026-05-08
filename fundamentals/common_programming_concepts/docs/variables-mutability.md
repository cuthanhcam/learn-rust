# Variables and Mutability in Rust

Rust enforces immutability by default, which is a core part of its safety model.

Instead of allowing unrestricted mutation like many languages, Rust forces you to **explicitly declare when and where state can change**.

This leads to:

- safer programs
- fewer hidden bugs
- predictable behavior
- better concurrency guarantees

Understanding variables, mutability, constants, and shadowing is essential before moving into ownership and borrowing.

---

## Table of Contents

1. Immutable Variables
2. Mutable Variables
3. Constants
4. Shadowing
5. Scope Shadowing
6. Shadowing for Type Transformation
7. Mutability and Ownership
8. Practical Shadowing Pattern
9. `mut` vs Shadowing
10. Summary

---

## Immutable Variables

```rust
let x = 5;
// x = 10; // compile-time error
```

In Rust, variables are immutable by default.

This means:

- once assigned, a value cannot change
- any reassignment is rejected at compile time

### Why this matters

Immutability removes entire classes of bugs:

- accidental overwrites
- unpredictable state changes
- race conditions in concurrent code

Instead of discovering errors at runtime, Rust enforces correctness during compilation.

---

## Mutable Variables

```rust
let mut counter = 0;
counter += 1;
counter += 5;
```

To allow modification, you must explicitly opt in using `mut`.

### Key idea

Rust does NOT remove safety when you use `mut`.
It only makes mutation **visible and intentional**.

Even with `mut`, Rust still enforces:

- ownership rules
- borrowing rules
- type safety

### Example with heap data

```rust
let mut message = String::from("Hello");
message.push_str(", Rust!");
```

Mutation is allowed, but still fully controlled by Rust’s ownership system.
