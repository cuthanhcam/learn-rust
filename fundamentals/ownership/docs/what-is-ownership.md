# What Is Ownership in Rust?

Ownership is the foundation of Rust's memory model. It is the rule system that lets Rust manage memory safely without a garbage collector.

If you understand ownership, the rest of Rust becomes much easier to read:

- why values move instead of being copied automatically
- why some functions take ownership and others borrow
- why Rust prevents use-after-free bugs at compile time
- why strings behave differently from simple stack values

The goal of this chapter is not just to memorize the rules. It is to build a mental model for how Rust stores, moves, and drops values.

---

## Table of Contents

1. Ownership in One Sentence
2. Stack and Heap Memory
3. The Three Ownership Rules
4. Variable Scope and Lifetime of a Value
5. The String Type and Heap Allocation
6. Moving Values and Why Copies Are Different
7. Ownership and Function Calls
8. Return Values and Ownership Transfer
9. Common Mistakes to Avoid
10. Summary

---

## Ownership in One Sentence

In Rust, every value has exactly one owner at a time, and when the owner goes out of scope, the value is dropped automatically.

This one rule explains a lot of Rust's behavior.

### Why Rust does this

Rust wants to guarantee memory safety at compile time.

That means Rust must be able to answer questions such as:

- Who is responsible for cleaning up this value?
- Can this value still be used here?
- Is another part of the program still relying on it?

Ownership gives Rust a clear answer to all of those questions.

---

## Stack and Heap Memory

Rust often talks about the stack and the heap because ownership behaves differently depending on where data lives.

### Stack

The stack is fast and organized. It works well for values whose size is known at compile time.

Examples:

- integers
- Booleans
- floating-point numbers
- fixed-size tuples and arrays with known size

Stack values are copied quickly because they are small and have a predictable layout.

### Heap

The heap is used for data whose size may change or is not known ahead of time.

Examples:

- `String`
- `Vec<T>`
- hash maps
- many dynamically sized structures

Heap allocation is more flexible, but it requires bookkeeping:

- allocate space
- track the pointer to that space
- free the space when it is no longer needed

### Why this matters for ownership

Ownership is mostly invisible for simple stack values, but it becomes very important for heap values because Rust must prevent multiple parts of the program from assuming they own the same allocation.

---

## The Three Ownership Rules

Rust ownership is usually summarized by three rules:

1. Each value has a single owner.
2. A value can have only one owner at a time.
3. When the owner goes out of scope, the value is dropped.

These rules are simple, but they drive most of Rust's safety guarantees.

### Example

```rust
{
    let message = String::from("hello");
    println!("{message}");
}
```

When the block ends, `message` goes out of scope and Rust automatically frees the memory used by the `String`.

### Important consequence

Because cleanup is automatic, Rust does not need a garbage collector.

Instead, it uses scope and ownership information to know exactly when cleanup should happen.

---

## Variable Scope and Lifetime of a Value

A variable's scope is the region of code where it is valid.

```rust
{
    let x = 5;
    println!("{x}");
}
```

In this example, `x` exists only inside the block.

When a value is owned by a variable, the end of the variable's scope usually means the end of the value's lifetime as well.

### Scope is not the same as lifetime annotations

In chapter 4.1, think about lifetime in the everyday sense:

- the value exists while its owner exists
- once the owner ends, the value is dropped

Later, Rust lifetime annotations will describe relationships between references. That is a different topic, but the intuition starts here.

---

## The String Type and Heap Allocation

`String` is a good example because it stores text on the heap and can grow.

```rust
let mut s = String::from("hello");
s.push_str(", world");
```

A string literal like `"hello"` is different from a `String`.

### String literal

- stored in the program binary
- fixed size
- immutable
- often represented as `&str`

### `String`

- heap allocated
- growable
- owned by the variable
- cleaned up automatically when it goes out of scope

### Why Rust uses `String` instead of only string literals

Because many real programs need to build or modify text at runtime.

Examples:

- reading input from a user
- assembling file paths
- formatting messages
- parsing data from files or the network

A `String` makes those cases possible while still being memory safe.

---

## Moving Values and Why Copies Are Different

Ownership becomes visible when values are assigned or passed around.

### For simple stack values

```rust
let x = 5;
let y = x;
```

Here, `x` is still valid after the assignment because `i32` implements `Copy`.

Rust duplicates the value instead of moving it.

### For heap-owning values

```rust
let s1 = String::from("hello");
let s2 = s1;
```

In this case, ownership moves from `s1` to `s2`.

After the move:

- `s2` owns the `String`
- `s1` is no longer valid

This prevents two variables from trying to clean up the same heap allocation.

### Why moves exist

If Rust allowed a naive bitwise copy of heap-owning values, both variables would point to the same allocation and both would try to free it.

That would cause a double free bug.

Rust avoids that by making ownership transfer explicit.

### Key distinction

- `Copy` types are duplicated implicitly
- non-`Copy` types are moved

This is one of the most important differences to internalize early.

---

## Ownership and Function Calls

When you pass a value into a function, ownership behavior depends on the argument type.

### Passing a copied value

```rust
fn takes_copy(value: i32) {
    println!("{value}");
}

let x = 10;
takes_copy(x);
println!("{x}");
```

`x` is still usable after the call because `i32` is copied.

### Passing an owned heap value

```rust
fn takes_ownership(text: String) {
    println!("{text}");
}

let s = String::from("hello");
takes_ownership(s);
```

Here, `s` is moved into the function.

After the call, the original variable can no longer be used.

### Why functions take ownership sometimes

A function may need to:

- store the value
- modify and return it
- take responsibility for cleaning it up
- consume the value as part of its job

If a function truly needs to own the value, taking ownership is the cleanest model.

---

## Return Values and Ownership Transfer

Functions can also return ownership.

```rust
fn give_ownership() -> String {
    String::from("hello")
}

let s = give_ownership();
```

The returned `String` is now owned by `s`.

### Returning a value after using it

```rust
fn take_and_return(text: String) -> String {
    println!("{text}");
    text
}

let s1 = String::from("hello");
let s2 = take_and_return(s1);
```

This pattern is useful when a function needs to inspect or transform a value but the caller should keep owning it afterward.

### Why this matters

Ownership can move into a function and then move back out.

That makes ownership flow explicit and predictable.

---

## Common Mistakes to Avoid

### Using a moved value

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}"); // error: s1 was moved
```

Once a value is moved, the original binding is invalid.

### Assuming all assignments copy

Assignments do not always copy.

They copy only when the type supports `Copy`.

### Thinking ownership means mutation

Ownership is about responsibility for a value, not whether the value can change.

A value can be owned, borrowed, immutable, or mutable. Those are related ideas, but they are not the same thing.

### Thinking `String` and `&str` are interchangeable

They are related, but not identical:

- `String` owns heap data
- `&str` borrows text

This difference becomes important in the next chapter on references and borrowing.

---

## Summary

Ownership is Rust's core memory management model.

The most important ideas are:

- every value has one owner
- ownership can move, but it cannot be duplicated by default
- values are dropped automatically when their owner goes out of scope
- stack values are often copied, while heap-owning values are usually moved
- function calls can transfer ownership in and out

If you understand these rules, you are ready for references, borrowing, and slices.

---

## Next Reading

- [References and Borrowing](./references-and-borrowing.md)
- [The Slice Type](./slice-type.md)

## Source Notes

The current crate at [src/main.rs](../src/main.rs) is still the starter binary. You can use it as the place to add small ownership examples as you work through the chapter.
