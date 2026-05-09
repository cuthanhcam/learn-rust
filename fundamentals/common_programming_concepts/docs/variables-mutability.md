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

---

## Constants

```rust
const MAX_PLAYERS: u8 = 4;
const SECONDS_PER_MINUTE: u32 = 60;
```

Constants represent values that:

- never change
- are computed at compile time
- are globally accessible (if needed)

### Key differences from variables

- Always immutable (no `mut`)
- Must have explicit types
- Inlined by compiler (no runtime cost)

### Use cases

- configuration values
- limits and thresholds
- mathematical constants

---

## Shadowing

```rust
let x = 5;
let x = x + 1;
let x = x * 2;
```

Shadowing means:

> creating a new variable with the same name

### Important distinction

Shadowing is NOT mutation.

| Feature      | mut | shadowing      |
| ------------ | --- | -------------- |
| Reuses value | Yes | Yes (as input) |
| New variable | No  | Yes            |
| Type change  | No  | Yes            |

### Why shadowing exists

It is used for:

- step-by-step transformations
- keeping variable names clean
- avoiding unnecessary temporary names

---

## Scope Shadowing

```rust
let x = 10;

{
    let x = x * 2;
    println!("{x}");
}

println!("{x}");
```

Rust uses lexical scoping:

- variables exist only inside their scope
- inner scopes can shadow outer variables

When the inner scope ends:

- the inner binding is dropped
- the outer binding becomes visible again

This enables isolated transformations without side effects.

---

## Shadowing for Type Transformation

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Shadowing allows changing the type of a variable.

This is NOT possible with `mut`:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // error: type mismatch
```

### Common real-world pattern

Shadowing is often used in pipelines:

```
raw input → cleaned input → parsed value → validated value
```

Each step creates a safer representation of the data.

---

## Mutability and Ownership

```rust
let mut message = String::from("Hello");
message.push_str(", Rust!");
```

Mutability does not bypass Rust’s ownership system.

Even mutable data must follow:

- single ownership rules
- borrowing rules
- no invalid references

### Key idea

> Mutation is allowed, but never unsafe.

---

## Practical Shadowing Pattern

```rust
let input = " 42 ";
let input = input.trim();
let input: i32 = input.parse().expect("Failed to parse");
let input = input * 2;
```

This pattern is common in real Rust code.

### Why it is preferred

- avoids temporary variable clutter
- keeps transformations linear
- improves readability
- keeps scope tight

---

## `mut` vs Shadowing

```rust
let mut value = 5;
value += 1;

let value = value.to_string();
```

### `mut`

- modifies same variable
- same type throughout lifetime
- used for incremental changes

### Shadowing

- creates new variable
- can change type
- used for transformations

---

## Summary

### Core principles

1. Variables are immutable by default
   → Rust favors safety over convenience

2. `mut` enables controlled mutation
   → mutation must always be explicit

3. Constants are compile-time values
   → no runtime cost

4. Shadowing creates new bindings
   → useful for transformations

5. Shadowing allows type changes
   → unlike `mut`

6. Rust enforces safety at compile time
   → fewer runtime errors

7. Idiomatic Rust prefers:
    - immutability by default
    - explicit mutation
    - small scopes
    - transformation via shadowing

---

## Reference

- [https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [https://doc.rust-lang.org/rust-by-example/variable.html](https://doc.rust-lang.org/rust-by-example/variable.html)
