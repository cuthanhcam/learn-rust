# Enums and Pattern Matching

This chapter explains how Rust uses enums and pattern matching to model state and branch safety.

Enums let you encode valid states in the type system. Pattern matching then forces you to handle every relevant case, which removes many logic bugs at compile time.

## Topics

- [Defining an Enum](./defining-an-enum.md)
- [The match Control Flow Construct](./match-control-flow.md)
- [Consise Control Flow with if let and let...else](concise-control-flow.md)

## Learning Goals

- Models finite state using enum variants
- Write exhaustive `match` expressions
- Destructure and use variant payload data
- Choose between `match`, `if let`, and `let...else`

## Common Pitfalls

- Hiding meaningful cases behind `_`
- Forgetting to update matches when adding a new variant
- Using `if let` where full `match` is clearer
- Moving enum-owned data unintentionally

## Source Demos

- [enums.rs](../src/enums.rs)
- [match_control_flow.rs](../src/match_control_flow.rs)
- [if_let.rs](../src/if_let.rs)

## Suggested Reading Order

1. Defining an Enum
2. The match Control Flow Construct
3. Concise Control Flow with if let and let...else
