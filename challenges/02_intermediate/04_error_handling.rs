// Challenge: Error Handling
// Complete each function according to its description.

use std::num::ParseIntError;

/// Exercise 1: Result
/// Write a function that divides two f64 numbers. 
/// If the denominator is 0.0, return `Err("Cannot divide by zero")`.
/// Otherwise, return `Ok(result)`.
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    todo!()
}

/// Exercise 2: The ? Operator
/// Parse a string into an i32 using `.parse::<i32>()`.
/// If it fails, propagate the error using the `?` operator.
pub fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Exercise 3: Multiple ? Operators
/// Parse two strings into i32s, add them, and return the result.
pub fn add_parsed(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Exercise 4: Panic!
/// Write a function that returns an element from an array slice by index.
/// If the index is out of bounds, manually `panic!` with the message "Index out of bounds!".
pub fn get_element(arr: &[i32], index: usize) -> i32 {
    todo!()
}

/// Exercise 5: unwrap_or
/// Try to parse a string to an i32. If it fails, use `.unwrap_or()` to return 0.
pub fn parse_or_zero(s: &str) -> i32 {
    todo!()
}

fn main() {
    println!("Run `rustc --test 04_error_handling.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err("Cannot divide by zero"));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_add_parsed() {
        assert_eq!(add_parsed("10", "20").unwrap(), 30);
        assert!(add_parsed("10", "abc").is_err());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_get_element_panic() {
        get_element(&[1, 2, 3], 5);
    }

    #[test]
    fn test_get_element() {
        assert_eq!(get_element(&[1, 2, 3], 1), 2);
    }

    #[test]
    fn test_parse_or_zero() {
        assert_eq!(parse_or_zero("100"), 100);
        assert_eq!(parse_or_zero("nope"), 0);
    }
}
