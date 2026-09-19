# 3. Common Programming Concepts

This chapter gives names to the everyday building blocks of a Rust program: values, variables, functions, decisions, and repetition. Rust syntax will look familiar after TypeScript, but its defaults are intentionally safer.

## Variables: immutable unless you say otherwise

```rust
let language = "Rust";
let mut score = 0;

score += 1;
// language = "Go"; // not allowed: `language` is not mutable
```

`let` creates a binding: a name that refers to a value. Most bindings are immutable. This prevents accidental changes and makes code easier to reason about. Use `mut` only when changing the value is part of the job.

### Shadowing is a new binding

```rust
let spaces = "   ";
let spaces = spaces.len(); // now `spaces` is a usize
```

The second `let` creates a new value with the same name. It is not mutation, so it may even have a different type. In TypeScript, this is closest to making a new `const` inside a narrower scope.

## Primitive values

Rust needs to know each value's size and meaning. The compiler can often infer the type, but type annotations make an important choice clear.

```rust
let signed: i32 = -42;
let unsigned: u32 = 42;
let price: f64 = 19.99;
let ready: bool = true;
let initial: char = 'R';
let pair: (i32, bool) = (7, false);
let days: [u8; 7] = [0; 7];
```

`i32`, `u32`, `f64`, `bool`, `char`, fixed-size arrays, and tuples store their data directly in the value. A local value normally lives in its function's stack frame. `char` is a Unicode scalar value and takes four bytes; it is not the same thing as a one-byte ASCII character.

## Functions and expressions

```rust
fn double(number: i32) -> i32 {
    number * 2
}

fn main() {
    let answer = double(21);
    println!("{answer}");
}
```

Parameters have types. `-> i32` says the function returns an `i32`. The last line has no semicolon, so it is an **expression** whose value becomes the return value. Add a semicolon and it becomes a statement that returns `()` instead.

```rust
let label = if score > 10 { "high" } else { "low" };
```

`if` is also an expression: both branches must return compatible types.

## Decisions and loops

```rust
let number = 6;

if number % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}

for item in [10, 20, 30] {
    println!("{item}");
}
```

Use `if` for a condition, `match` when selecting among well-defined shapes, `loop` when you need an explicit `break`, `while` for a condition checked each turn, and `for` to visit items from an iterator. Prefer `for` over indexing a collection when you simply need every item.

## What the machine sees

For a function such as `double(21)`, the CPU receives or loads the number into a register, multiplies it, and returns the result. In optimized code, the compiler may inline this tiny function, so no separate call happens at runtime. The Rust rules describe what your program means; the compiler is free to make it faster when the observable result stays the same.

## Common mistakes

- `"text"` is a string slice (`&str`), while `'x'` is one `char`.
- Array indexes start at zero. `items[items.len()]` is one past the end and panics.
- An integer literal such as `5` has its type inferred from context. Add `5_u64` or `let count: u64 = 5;` when it matters.
- Rust does not automatically turn a number into text. Use `number.to_string()` or formatting such as `format!("{number}")`.

## Try it now

Complete the first three files in [beginner challenges](../challenges/01_beginner/). Then build the tiny command-line greeter project in [Project 1](../challenges/04_projects/01_cli_greeter.md).
