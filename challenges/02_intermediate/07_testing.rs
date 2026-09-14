/// Rustonomicon Challenge: Testing
///
/// In Rust, testing is built-in. You don't need external libraries like Jest or Vitest.
/// Tests are just regular functions annotated with `#[test]`.
/// In this challenge, you will implement both the code and the tests!

/// Exercise 1: Basic assertion
/// Write a function `add_two` that takes an i32 and returns an i32.
/// Then write a test that verifies it works using `assert_eq!`.
/// TS analogy: `expect(addTwo(2)).toBe(4)`
pub fn add_two(a: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests_ex1 {
    use super::*;

    #[test]
    fn test_add_two() {
        todo!()
    }
}

/// Exercise 2: Testing with `assert!`
/// Write a function `is_even` that returns true if a number is even.
/// Test it using the `assert!` macro (which is for booleans).
/// TS analogy: `expect(isEven(4)).toBeTruthy()`
pub fn is_even(n: i32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_ex2 {
    use super::*;

    #[test]
    fn test_is_even() {
        todo!()
    }
}

/// Exercise 3: Testing for panics
/// Write a function `divide` that panics with "Divide by zero!" if the denominator is 0.
/// Write a test that verifies the panic using the `#[should_panic]` attribute.
/// TS analogy: `expect(() => divide(1, 0)).toThrow('Divide by zero!')`
pub fn divide(a: i32, b: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests_ex3 {
    use super::*;

    #[test]
    // Hint: Add an attribute here
    fn test_divide_by_zero() {
        todo!()
    }
}

/// Exercise 4: Testing with Result
/// Tests can return `Result<(), String>` instead of panicking.
/// This allows using the `?` operator in tests.
/// Write a function `parse_number` that parses a string to i32, returning a Result.
/// Then write a test that returns `Result<(), String>` and uses `?`.
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

#[cfg(test)]
mod tests_ex4 {
    use super::*;

    #[test]
    fn test_parse_number() -> Result<(), String> {
        todo!()
    }
}

/// Exercise 5: Custom failure messages
/// `assert!` and `assert_eq!` can take custom failure messages as additional arguments.
/// Write a function `greet` that returns "Hello, {name}".
/// Write a test for it that uses a custom failure message like "Greeting was incorrect!".
pub fn greet(name: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_ex5 {
    use super::*;

    #[test]
    fn test_greet() {
        todo!()
    }
}

/// Exercise 6: Testing private functions
/// In Rust, the `tests` module is a child module and can access private functions of its parent.
/// Write a private helper function `internal_calc` and write a test for it.
/// TS analogy: Testing unexported functions is tricky in TS, but trivial in Rust.
fn internal_calc(x: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests_ex6 {
    use super::*;

    #[test]
    fn test_internal_calc() {
        todo!()
    }
}

/// Exercise 7: Using `#[ignore]`
/// Some tests are very slow or require special setup.
/// Write a test that sleeps for 5 seconds and mark it with `#[ignore]`.
/// (You can run ignored tests with `cargo test -- --ignored` or `rustc --test foo.rs && ./foo --ignored`)
/// TS analogy: `test.skip(...)` or `test.todo(...)`
#[cfg(test)]
mod tests_ex7 {
    #[test]
    // Hint: Add the ignore attribute here
    fn very_slow_test() {
        todo!()
    }
}

/// Exercise 8: Integration-style test
/// Integration tests usually live in a separate `tests/` directory and can only access public APIs.
/// We'll simulate this by not importing private items.
/// Create a public struct `Config` with a constructor `new`.
/// Write a test that tests `Config::new` using only the public interface.
pub struct Config {
    pub max_connections: u32,
}

impl Config {
    pub fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests_ex8 {
    // Only import the public struct
    use super::Config;

    #[test]
    fn test_public_api() {
        todo!()
    }
}

fn main() {
    println!("Run 'rustc --test 07_testing.rs && ./07_testing' to test your solutions!");
}
