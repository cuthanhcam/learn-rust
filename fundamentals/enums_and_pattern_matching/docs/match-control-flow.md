# The match Control Flow Construct

`match` is Rust's primary pattern matching construct.

It compares a value against patterns and executes the arm that matches. Every arm must produce a compatible type for the overall expression.

## Why match Is Important

- it is exhaustive by default
- it supports rich patterns
- it pairs naturally with enums and `Option`/`Result`

## Patterns Used in This Lesson

- exact values (`1`, `2`, `3`)
- alternation (`1 | 2`)
- ranges (`3..=5`)
- wildcard (`_`)
- destructuring enum payloads (`Coin::Quarter(state)`)
- guards (`Some(value) if value > 0`)

## Exhaustiveness

Rust requires you to cover all possibilities. This helps you avoid silent logic gaps when you add enum variants later.

## When to Prefer match

- handling many cases
- needing variant payload destructuring
- requiring compile-time guarantee every case is addressed

## Common Pitfalls

- overusing wildcard arms early and losing intent
- duplicated logic across multiple arms instead of extracting helper functions
- assuming all arms can return different unrelated types

## Practice Tasks

1. Add a `Result`-based match that maps errors to user messages.
2. Use a guard arm (`if`) to separate positive and negative numbers.
3. Convert a nested `if` chain into one `match`.

## Demo File

See [match_control_flow.rs](../src/match_control_flow.rs).
