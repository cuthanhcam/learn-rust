# Rust Comments and Documentation

Writing good Rust code is not only about making it compile. Clear comments and useful docs help other people (including future you) understand intent, behavior, and safe usage.

This tutorial covers comment types, doc comment patterns, and practical documentation workflows with Cargo.

## Comment Types in Rust

### Line Comments (`//`)

Use line comments for short, local context.

```rust
// Retry at most 5 times to avoid an endless network loop.
let retry_limit = 5;
```

### Block Comments (`/* ... */`)

Use block comments for longer notes or temporary investigation notes.

```rust
/*
Temporary note:
This path is kept for backwards compatibility with v1 clients.
*/
let use_legacy_parser = true;
```

Rust supports nested block comments:

```rust
/* outer comment
   /* nested comment */
*/
```

## Documentation Comments

Rust can generate API docs directly from your code comments via `cargo doc`.

### Outer Doc Comments (`///`)

Use for items such as functions, structs, enums, traits, and modules.

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let value = add(2, 3);
/// assert_eq!(value, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

### Inner Doc Comments (`//!`)

Use at the top of a module or crate file to describe the entire scope.

```rust
//! Utilities for parsing and validating user input.
```

### Block Doc Comments (`/**` and `/*!`)

These are valid, but less common than `///` and `//!`:

```rust
/** Public API helper. */
pub fn helper() {}

/*! Crate-level documentation. */
```

## Recommended Doc Sections

Common Markdown sections in Rust docs:

- `# Examples`
- `# Panics`
- `# Errors`
- `# Safety` (for `unsafe` APIs)

Example with `Result` and error behavior:

````rust
/// Parses a port number from a string.
///
/// # Errors
///
/// Returns `Err` if the string is not a valid `u16` value.
///
/// # Examples
///
/// ```
/// let port = parse_port("8080").unwrap();
/// assert_eq!(port, 8080);
/// ```
pub fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse::<u16>()
}
````

## Doc Tests (Doctests)

Code blocks in doc comments are tested by default.

Run all tests, including doctests:

```bash
cargo test
```

Generate and open documentation locally:

```bash
cargo doc --open
```

If you want to show code that should not run, mark the block:

````rust
/// ```ignore
/// // Example pseudo-code for explanation only.
/// ```
````

## Best Practices

1. Document public APIs with `///`.
2. Explain intent and constraints, not obvious syntax.
3. Keep comments in sync with code changes.
4. Prefer clear naming to reduce unnecessary comments.
5. Add runnable examples for key functions and types.

## Quick Reference

| Type              | Syntax      | Typical Use                                |
| ----------------- | ----------- | ------------------------------------------ |
| Line comment      | `//`        | Short notes and inline intent              |
| Block comment     | `/* ... */` | Longer notes or temporary investigation    |
| Outer doc comment | `///`       | Document functions, structs, enums, traits |
| Inner doc comment | `//!`       | Document module or crate at file top       |
