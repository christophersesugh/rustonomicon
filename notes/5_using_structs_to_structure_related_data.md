# 5. Structs: Group Related Data

A struct is a custom type with named fields. Use one when several values describe one thing. It is the Rust equivalent of a small TypeScript object type, but a Rust struct has a fixed memory layout known to the compiler.

## Define and create a struct

```rust
#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        name: String::from("Amina"),
        active: true,
        sign_in_count: 1,
    };

    user.sign_in_count += 1;
    println!("{user:?}");
}
```

`#[derive(Debug)]` asks the compiler to generate a developer-friendly way to print the value with `{:?}`. The whole `user` binding must be mutable before any field can change.

## A struct owns its fields

`User` owns its `String`. The three-word `String` handle is stored inside the `User` value; the characters are in a heap allocation. When `user` leaves scope, Rust drops its fields, and the `String` frees that allocation.

```text
stack                              heap
┌────────────────────────────────┐ ┌──────────────────┐
│ user                            │ │ A m i n a        │
│ name: pointer, length, capacity │─▶│ string bytes     │
│ active: true                    │ └──────────────────┘
│ sign_in_count: 2                │
└────────────────────────────────┘
```

If the struct instead held `name: &str`, it would borrow text owned somewhere else. Chapter 10 explains how Rust tracks that safely with lifetimes.

## Methods put behavior next to data

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
```

`&self` is short for `self: &Rectangle`: borrow the rectangle without moving it. `&mut self` borrows it exclusively so a method can change it. Plain `self` consumes the whole value and is useful when the method deliberately turns it into something else.

`Rectangle::square(5)` is an **associated function**. It has no `self`, so call it with `::`, like a static factory method in TypeScript.

### Method-call convenience

When you write `rectangle.area()`, Rust automatically borrows `rectangle` to meet an `&self` receiver when it can. It does not ignore ownership rules; it merely inserts the obvious `&` or `&mut`.

## Three struct forms

```rust
struct Point { x: i32, y: i32 }             // named fields
struct Color(u8, u8, u8);                   // tuple struct
struct Marker;                              // unit-like struct, no data
```

Tuple structs are useful when the values belong together but field names would add no clarity. A unit-like struct can carry behavior through trait implementations even though it needs no per-value data.

## Useful derived traits

Add only traits that make sense for your type:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Settings {
    dark_mode: bool,
}
```

- `Debug`: print for developers.
- `Clone`: explicitly duplicate every cloneable field.
- `PartialEq` / `Eq`: compare values using `==`.
- `Default`: provide `Settings::default()`.

The derive macro writes the predictable field-by-field implementation at compile time. It does not add a runtime framework.

## Try it now

Work through [the struct exercises](../challenges/02_intermediate/01_structs.rs), then make the task tracker in [Project 3](../challenges/04_projects/03_task_tracker.md).
