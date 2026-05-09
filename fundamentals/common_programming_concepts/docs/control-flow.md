# Control Flow in Rust

Control flow decides what code runs and how many times it runs.

The two main building blocks in this chapter are branching with `if` expressions and repetition with loops.

The demo code for this lesson lives in [control_flow.rs](../src/control_flow.rs).

---

## Table of Contents

1. if Expressions
2. else if Chains
3. if in a let Statement
4. loop
5. while
6. for
7. Loop Labels
8. Summary
9. Reference

---

## if Expressions

An `if` expression runs one block when a condition is true and another when it is false.

```rust
if number < 5 {
	println!("condition was true");
} else {
	println!("condition was false");
}
```

Important rules:

- the condition must be a `bool`
- Rust does not implicitly convert integers or strings into booleans
- the arms of the expression must produce compatible types when used as a value

### Why this matters

Rust keeps branching explicit so the compiler can reason about the code at compile time.

---

## else if Chains

When you need more than two branches, use `else if`.

```rust
if number % 4 == 0 {
	println!("number is divisible by 4");
} else if number % 3 == 0 {
	println!("number is divisible by 3");
} else if number % 2 == 0 {
	println!("number is divisible by 2");
} else {
	println!("number is not divisible by 4, 3, or 2");
}
```

Rust runs only the first branch that evaluates to true.

### Practical advice

If you have many branches, consider whether a `match` expression would be clearer.

---

## if in a let Statement

Because `if` is an expression, it can be used on the right side of `let`.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

The value of the whole expression is bound to `number`.

Both branches must produce the same type.

---

## loop

`loop` repeats forever until you stop it with `break`.

```rust
loop {
	println!("again!");
}
```

You can also use `break` with a value:

```rust
let result = loop {
	if counter == 10 {
		break counter * 2;
	}
};
```

### break and continue

- `break` exits the loop immediately
- `continue` skips the rest of the current iteration and starts the next one

---

## while

`while` runs a loop while a condition is true.

```rust
while number != 0 {
	println!("{number}!");
	number -= 1;
}
```

This is a compact way to express condition-based repetition.

It is especially helpful when the loop depends on state that changes each time through the body.

---

## for

`for` is the most common way to iterate over a collection or a range.

```rust
for element in a {
	println!("the value is: {element}");
}
```

You can also use it with ranges:

```rust
for number in (1..4).rev() {
	println!("{number}!");
}
```

### Why `for` is preferred

- it avoids manual index management
- it reduces the risk of out-of-bounds errors
- it is concise and idiomatic

---

## Loop Labels

Labels let you control which loop `break` or `continue` applies to when loops are nested.

```rust
'counting_up: loop {
	loop {
		break 'counting_up;
	}
}
```

Use labels when nested loops would otherwise make control flow ambiguous.

---

## Summary

1. `if` expressions branch based on a boolean condition
   -> Rust does not coerce non-booleans

2. `else if` adds more branches
   -> only the first true branch runs

3. `if` can be used as an expression
   -> that is why it works inside `let`

4. `loop` repeats until broken
   -> it can also return a value

5. `while` repeats while a condition stays true
   -> useful for countdowns and state-driven loops

6. `for` is the idiomatic choice for collections and ranges
   -> safer than manual indexing

7. Loop labels disambiguate nested loops
   -> they target the intended loop explicitly

---

## Reference

- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example: Control Flow](https://doc.rust-lang.org/rust-by-example/flow_control.html)
