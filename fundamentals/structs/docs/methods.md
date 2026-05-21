# Methods and Associated Functions

This section explains how to move behavior into `impl` blocks so types and operations stay together.

## Method Receivers

- `&self`: immutable borrow, read-only behavior
- `&mut self`: mutable borrow, modifies instance
- `self`: consumes the instance

## Associated Functions

Functions in `impl` without a `self` receiver are associated functions.

Common use:

- constructors such as `Rectangle::square(size)`

## Extra Notes

- methods can have the same name as fields (`rect.width()` vs `rect.width`)
- Rust performs automatic referencing/dereferencing for method calls

## Demo File

See [methods.rs](../src/methods.rs).
