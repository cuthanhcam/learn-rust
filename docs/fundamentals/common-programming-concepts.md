# Common Programming Concepts

This chapter introduces the foundational concepts used in almost every Rust program.

These concepts establish the mental models required before learning:

- ownership
- borrowing
- lifetimes
- structs
- enums
- concurrency
- async Rust

---

# Topics

## Variables and Mutability

Learn:

- immutable bindings and compile-time safety
- mutable variables with `mut`
- constants and explicit types
- shadowing as a new binding, not mutation
- scope rules and type-changing transformations

→ [Detailed Notes](../../fundamentals/common_programming_concepts/docs/variables-mutability.md)

→ [Source Demo](../../fundamentals/common_programming_concepts/src/variables_mutability.rs)

---

## Data Types

Learn:

- scalar types, including integers, floats, Booleans, and characters
- compound types, especially tuples and arrays
- type annotations for ambiguous cases like parsing
- fixed-size collections and runtime bounds checks

→ [Detailed Notes](../../fundamentals/common_programming_concepts/docs/data-types.md)

→ [Source Demo](../../fundamentals/common_programming_concepts/src/data_types.rs)

---

## Functions

Learn:

- defining and calling functions
- typed parameters
- statements versus expressions
- return values and early returns

→ [Detailed Notes](../../fundamentals/common_programming_concepts/docs/functions.md)

→ [Source Demo](../../fundamentals/common_programming_concepts/src/functions.rs)

---

## Comments

Learn:

- line comments with `//`
- trailing comments
- block comments with `/* */`
- documentation comments with `///`

→ [Detailed Notes](../../fundamentals/common_programming_concepts/docs/comments.md)

→ [Source Demo](../../fundamentals/common_programming_concepts/src/comments.rs)

---

## Control Flow

Learn:

- `if` expressions and `else if` chains
- using `if` inside a `let` statement
- `loop`, `while`, and `for`
- loop return values and loop labels

→ [Detailed Notes](../../fundamentals/common_programming_concepts/docs/control-flow.md)

→ [Source Demo](../../fundamentals/common_programming_concepts/src/control_flow.rs)
