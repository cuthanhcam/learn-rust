# Defining and Instantiating Structs

This section introduces struct syntax and the common patterns for creating values from a struct definition.

## Core Concepts

- Named structs
- Tuple structs
- Unit-like structs
- Field init shorthand
- Struct update syntax

## Named Structs

Named structs give each field semantic meaning, which improves readability and prevents index confusion common with tuples.

## Field Init Shorthand

When parameter names match field names, you can write:

```rust
User { email, username, active: true, sign_in_count: 1 }
```

This keeps constructors concise.

## Struct Update Syntax

`..existing` copies or moves remaining fields from another instance.

Important ownership rule:

- moved fields (such as `String`) make the source instance partially unusable
- `Copy` fields (such as `u64`, `bool`) remain usable

## Tuple and Unit-like Structs

Tuple structs provide type meaning without field names.
Unit-like structs are useful marker types when no data storage is needed.

## Demo File

See [defining.rs](../src/defining.rs).
