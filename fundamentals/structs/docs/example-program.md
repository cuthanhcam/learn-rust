# Example Program Using Structs

This section shows a practical refactor path from raw values to a domain struct.

## Refactor Path

1. Start with separate width and height variables.
2. Group them into a tuple.
3. Replace tuple indices with a `Rectangle` struct.

The result is clearer signatures and more maintainable code.

## Debugging Struct Values

`#[derive(Debug)]` enables `{:?}` and `{:#?}` output.

`dbg!` is useful for quickly inspecting intermediate expressions and values while preserving ownership flow.

## Demo File

See [example.rs](../src/example.rs).
