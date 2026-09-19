# 10. Generics, Traits, and Lifetimes

These three features let Rust write reusable code without giving up the exact safety checks that make it fast.

- **Generics** say “this code works for many concrete types.”
- **Traits** say “this type promises these capabilities.”
- **Lifetimes** say “this reference cannot outlive the data it points at.”

## Generics: a placeholder for a type

Instead of writing one `largest_i32` and one `largest_char`, describe the shared operation once:

```rust
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut items = items.iter();
    let mut largest = items.next()?;

    for item in items {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}
```

`T` is a type parameter. `T: PartialOrd` means a `T` must support comparisons such as `>`. `&[T]` borrows a slice, and `Option<&T>` either returns a reference to an item already in that slice or returns `None` for an empty slice. No item is cloned or moved.

### Behind the scenes: monomorphization

At compile time, Rust makes specialized versions of generic code for each concrete type it uses. A call with `i32` gets machine code for `i32`; a call with `char` gets machine code for `char`. This is called **monomorphization**. You get reuse in source code without runtime type checks in the common case.

## Traits: capabilities a type can provide

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

fn announce(item: &impl Summary) {
    println!("New: {}", item.summarize());
}
```

A trait is similar to a TypeScript interface, but it can include default method bodies and Rust verifies implementations during compilation. `announce` accepts a borrowed value of any type that implements `Summary`.

The longer equivalent is `fn announce<T: Summary>(item: &T)`. Use the short `impl Trait` form for simple parameters; use a named `T` when the same type appears in more than one place.

### Default methods

```rust
trait Greet {
    fn name(&self) -> &str;

    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}
```

Implementers only need to provide `name` to receive the default `greet`. The generated call is still ordinary statically dispatched code when the concrete type is known.

## Trait bounds describe requirements

```rust
fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("{} ({item})", item.summarize());
}
```

The `where` form keeps long requirements readable. It is a compile-time contract, not a runtime `instanceof` test.

## Lifetimes: references need a valid target

References are pointers with a promise: the pointed-to value will remain alive while the reference is used. Most of the time Rust infers that promise. You write lifetime annotations only when the relationship is unclear to the compiler.

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

`'a` does **not** make either string live longer. It says the returned reference will live no longer than the shorter of `left` and `right`. The caller must already keep both inputs alive long enough.

```text
owner's value:  ─────────────────────────── alive ───
borrow (`&str`):       ───── valid use ─────
```

The borrow is a view into someone else's data. Rust prevents the owner from disappearing, moving in a conflicting way, or being mutably changed while that view is still needed.

### Lifetimes in structs

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

This struct does not own `part`; it stores a pointer and length pointing into another string. The annotation tells the compiler an `ImportantExcerpt` cannot outlive that source text. If you want independent ownership, store `String` instead.

## Static versus dynamic dispatch

`&impl Summary` and `<T: Summary>` use **static dispatch**: the compiler knows the concrete type and can specialize the call. `&dyn Summary` is a **trait object** and uses dynamic dispatch through a vtable. It is useful when one collection must hold different concrete types, but it adds indirection and has object-safety rules. See [Chapter 18](./18_object_oriented_programming_features.md) after the basics feel solid.

## Common mistakes

- Adding a lifetime annotation cannot fix a dangling reference. Own the data with `String` or change where it is created.
- `Clone` is not a substitute for understanding ownership; it may copy heap data.
- A trait bound only promises methods. It does not convert one type into another.
- Do not add generic parameters “just in case.” Start concrete and generalize when repetition proves the need.

## Try it now

Complete [traits](../challenges/02_intermediate/05_traits.rs), [generics](../challenges/02_intermediate/06_generics.rs), and [lifetimes](../challenges/03_advanced/01_lifetimes.rs) in that order.
