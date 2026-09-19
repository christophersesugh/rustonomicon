# 13. Closures and Iterators

Closures are small functions that can remember nearby values. Iterators are lazy objects that visit values one at a time. Together they let you express data transformations clearly without giving up Rust's ownership checks.

## Closures: functions that can capture

```rust
let tax_rate = 0.075;
let with_tax = |price: f64| price * (1.0 + tax_rate);

assert_eq!(10.75, with_tax(10.0));
```

The closure uses `tax_rate` even though it is not a parameter. Rust examines how the closure uses captured values and chooses the least restrictive capture:

| How the closure uses a value | Trait it can implement | What happens |
| --- | --- | --- |
| Reads it | `Fn` | Shared borrow; call many times. |
| Changes it | `FnMut` | Mutable borrow; the closure binding must be `mut`. |
| Moves it out | `FnOnce` | Consumes captured data; call at most once. |

```rust
let name = String::from("Amina");
let announce = move || println!("Welcome, {name}!");
announce();
```

`move` transfers `name` into the closure. This is commonly necessary before sending a closure to another thread, because the original stack frame might disappear before that thread runs.

## A closure has a hidden type

Every closure has a distinct compiler-generated type containing its captured fields. The first example acts roughly like a tiny struct holding a reference to `tax_rate` plus a call implementation. Rust knows that concrete type at compile time, so calling a closure is usually as direct as calling a normal function.

You normally let type inference handle closure types. When accepting one as a parameter, use a trait bound:

```rust
fn apply_twice<F>(value: i32, mut transform: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    transform(transform(value))
}
```

## Iterators: a plan for visiting values

```rust
let values = vec![1, 2, 3];
let mut iterator = values.iter();

assert_eq!(Some(&1), iterator.next());
assert_eq!(Some(&2), iterator.next());
assert_eq!(Some(&3), iterator.next());
assert_eq!(None, iterator.next());
```

An iterator remembers where it is. Each `next()` mutates that state and returns `Some(item)` until it returns `None`.

Choose the iterator based on ownership:

| Call | Item type for `Vec<T>` | Meaning |
| --- | --- | --- |
| `.iter()` | `&T` | Borrow each item; keep the vector. |
| `.iter_mut()` | `&mut T` | Mutably borrow each item; change them in place. |
| `.into_iter()` | `T` | Move items out; consume the vector. |

## Adapters are lazy; consumers do the work

```rust
let values = vec![1, 2, 3, 4];
let doubled_evens: Vec<i32> = values
    .iter()
    .copied()
    .filter(|number| number % 2 == 0)
    .map(|number| number * 2)
    .collect();

assert_eq!(vec![4, 8], doubled_evens);
```

`filter` and `map` are **adapters**: they create a new iterator plan but do not loop yet. `collect` is a **consumer**: it asks for every result and builds the `Vec`. This laziness avoids intermediate vectors.

Behind the scenes, the compiler can often turn this pipeline into a loop very similar to a hand-written `for` loop. The closure and iterator state may live in registers or on the stack; it does not require a JavaScript-style runtime iterator object.

## A custom iterator

```rust
struct CountToThree {
    current: u8,
}

impl Iterator for CountToThree {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 3 {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}
```

`Iterator` requires only `next`. The standard library builds useful methods like `map`, `filter`, `zip`, `sum`, and `collect` on top of it.

## Common mistakes

- A closure that mutates captured state needs `let mut closure = ...` when called directly.
- Iterator adapters do nothing until a consumer such as `collect`, `sum`, `for_each`, or a `for` loop drives them.
- `.into_iter()` takes ownership. Pick `.iter()` when you still need the collection afterward.
- A chained iterator error often comes from whether an item is `T`, `&T`, or `&mut T`. Read the item type in the compiler message.

## Try it now

Finish [closures](../challenges/03_advanced/02_closures.rs) before [iterators](../challenges/03_advanced/03_iterators.rs). Then add a filter and summary to the task tracker project.
