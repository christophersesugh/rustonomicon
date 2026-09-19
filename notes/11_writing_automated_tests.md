# 11. Writing Automated Tests

A test is a small program that asks one clear question: “given this input, does my code produce the expected result?” Tests make refactoring less scary because the computer checks behavior you might forget to check manually.

## Write your first unit test

Put a `#[cfg(test)]` module beside the code it tests:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(4, add(2, 2));
    }
}
```

- `#[cfg(test)]` includes this module only when compiling tests.
- `#[test]` tells the test runner to call this function.
- `assert_eq!(expected, actual)` fails with a useful comparison when the values differ.

The test runner gives each test a fresh stack frame. Values created in one test are not shared with another unless you deliberately use files, environment variables, global state, or network resources.

## The assertions you will use most

```rust
assert!(is_valid("Ada"));
assert_eq!(3, count_words("one two three"));
assert_ne!(0, total);
```

Use `assert!` for a boolean condition, `assert_eq!` for equal values, and `assert_ne!` for different values. All three accept an optional format string for a better failure message:

```rust
assert!(age >= 18, "expected an adult, got {age}");
```

The compared values need `Debug` so Rust can print them on failure. Add `#[derive(Debug)]` to your own structs when a test needs to show them.

## Test errors and panics deliberately

```rust
/// Divides two numbers, rejecting a zero divisor.
pub fn divide(left: i32, right: i32) -> Result<i32, &'static str> {
    if right == 0 { Err("cannot divide by zero") } else { Ok(left / right) }
}

#[test]
fn rejects_zero() {
    assert_eq!(Err("cannot divide by zero"), divide(10, 0));
}
```

Prefer testing a returned `Result` for expected bad input. Use `#[should_panic]` only when a panic is really part of the API, such as indexing outside a fixed array.

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn indexing_past_the_end_panics() {
    let values = [1, 2];
    let _ = values[99];
}
```

Tests can return `Result<(), E>` too. This is handy when `?` keeps setup readable.

```rust
#[test]
fn parses_a_number() -> Result<(), std::num::ParseIntError> {
    let number: i32 = "42".parse()?;
    assert_eq!(42, number);
    Ok(())
}
```

## Unit tests and integration tests

**Unit tests** live beside the code and may inspect private details. They answer “does this small piece work?”

**Integration tests** go in a top-level `tests/` directory and can use only the library's public API. They answer “can another crate use my library correctly?”

```text
my_project/
├── src/
│   └── lib.rs
└── tests/
    └── search_api.rs
```

An integration test imports your package by its crate name, just like a real user. Binary-only packages are harder to integration-test, another reason to keep reusable behavior in `src/lib.rs`.

## Run only what you need

| Command | What it does |
| --- | --- |
| `cargo test` | Run the package's unit, integration, and documentation tests. |
| `cargo test name_part` | Run tests whose names contain `name_part`. |
| `cargo test --test search_api` | Run one integration-test file. |
| `cargo test -- --show-output` | Show `println!` output even for passing tests. |
| `cargo test -- --test-threads=1` | Run serially. Useful when tests share an external resource. |
| `cargo test -- --ignored` | Run tests marked `#[ignore]`. |
| `cargo test -- --include-ignored` | Run normal and ignored tests. |

By default, the runner captures output and may run tests in parallel. A test that relies on another test's order is a bug: make each test create the data it needs.

## A simple testing habit

1. Describe a behavior in one sentence.
2. Write the smallest failing test for it.
3. Implement only enough code to pass.
4. Add an edge case before moving on.
5. Run `cargo fmt` and `cargo test`.

For practice, complete [the testing challenge](../challenges/02_intermediate/07_testing.rs), then add tests to your [minigrep project](./12_io_project.md).
