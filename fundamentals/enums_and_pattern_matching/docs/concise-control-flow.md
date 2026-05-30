# Concise Control Flow with if let and let...else

`if let` and `let...else` are compact alternatives to full `match` when you only care about one success pattern.

## if let

Use `if let` when:

- you want to handle one specific pattern
- fallback logic is simple

Example shape:

```rust
if let Some(value) = option {
    // use value
} else {
    // fallback
}
```

## let...else

Use `let...else` when:

- successful destructuring is required before continuing
- failure should exit early (`return`, `break`, `continue`, or panic)

Example shape:

```rust
let Some(value) = option else {
    return;
};
```

This pattern keeps the happy path flat and readable.

## Quick Decision Guide

- use `match` for many branches or full coverage
- use `if let` for one interesting branch
- use `let...else` to exit early and keep the success path linear

## Common Pitfalls

- forcing complex branching into `if let`
- forgetting that `let...else` failure branch must diverge
- repeating heavy fallback logic in many `if let` blocks

## Practice Tasks

1. Refactor one `match` into `if let` and keep behavior identical.
2. Parse optional input with `let...else` and early return on `None`.
3. Compare readability of all three forms on the same example.

## Demo File

See [if_let.rs](../src/if_let.rs).
