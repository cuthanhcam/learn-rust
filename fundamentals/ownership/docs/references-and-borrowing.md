# References and Borrowing in Rust

References are Rust's answer to a common problem: how do you let code use a value without giving away ownership?

Borrowing lets a function or expression inspect or modify data temporarily while the original owner keeps control.

This chapter builds directly on ownership. If ownership says "who owns the value?", borrowing says "who is allowed to use it right now?"

---

## Table of Contents

1. Why Borrowing Exists
2. Immutable References
3. Mutable References
4. Reference Rules
5. Borrowing in Function Parameters
6. Returning References Carefully
7. Dangling References
8. Summary

---

## Why Borrowing Exists

Ownership is safe, but if every function had to take ownership of every value, programs would become awkward fast.

Borrowing solves that by allowing temporary access.

### Example use cases

- reading text without consuming it
- checking a value before deciding what to do with it
- mutating a data structure in place without copying it
- passing large values to helper functions efficiently

Borrowing keeps ownership in one place while allowing controlled access elsewhere.

---

## Immutable References

An immutable reference is a read-only view into a value.

```rust
let s = String::from("hello");
let len = s.len();
```

In many real programs, helper functions take `&T` when they only need to inspect data.

### What immutable borrowing allows

- multiple readers at the same time
- no transfer of ownership
- no mutation through the reference

### Why this is useful

You can share access freely as long as nobody is trying to change the value through that reference.

---

## Mutable References

A mutable reference allows controlled modification.

```rust
let mut s = String::from("hello");

let r = &mut s;
r.push_str(", world");
```

Mutable borrowing is stricter because changing data while others are reading it can create bugs.

### Rust's main rule for mutable references

At any point in time, you can have either:

- one mutable reference
- or any number of immutable references

Not both at the same time.

That rule prevents aliasing bugs where one part of the program changes data while another part assumes it is stable.

---

## Reference Rules

Rust enforces several practical rules around references:

- references must always be valid
- references cannot outlive the data they point to
- mutable and immutable borrows cannot conflict
- the compiler tracks borrow scope for you

These rules are the reason Rust can reject many bugs before the program runs.

---

## Borrowing in Function Parameters

Functions often take references when they only need access, not ownership.

```rust
fn calculate_length(text: &String) -> usize {
    text.len()
}
```

This lets the caller keep ownership after the function returns.

### Why this pattern matters

- fewer unnecessary moves
- better performance for large values
- clearer API intent

A function signature communicates whether the function consumes data or only borrows it.

---

## Returning References Carefully

Returning a reference is possible only when Rust can prove the referenced value will still exist.

That is why some function signatures are easy and others are rejected by the compiler.

A reference must never point to data that has already been dropped.

---

## Dangling References

A dangling reference points to memory that is no longer valid.

Rust prevents this by refusing code that would create a reference to a value that goes out of scope too soon.

This is one of the strongest examples of Rust's compile-time memory safety.

---

## Summary

Borrowing lets you use data without taking ownership.

The main ideas are:

- `&T` means shared, read-only access
- `&mut T` means exclusive, writable access
- references must always stay valid
- Rust prevents conflicting borrows and dangling references

If ownership is about responsibility, borrowing is about temporary access.
