// Challenge: Functions
// Complete each function according to its description.

/// Exercise 1: No return
/// Create a function `say_hello` that takes no arguments and returns nothing (unit type).
/// It just needs to exist for the test to compile.
pub fn say_hello() {
    todo!()
}

/// Exercise 2: Parameters and Return
/// Return the sum of a and b.
pub fn add(a: i32, b: i32) -> i32 {
    todo!()
}

/// Exercise 3: Early Return
/// If `n` is negative, return 0 early. Otherwise return `n * 2`.
pub fn double_if_positive(n: i32) -> i32 {
    todo!()
}

/// Exercise 4: Expression vs Statement
/// This function should use an expression block to calculate a value.
/// Calculate `(x + 5) * 2` inside a block assigned to a variable, then return it.
pub fn block_expression(x: i32) -> i32 {
    todo!()
}

/// Exercise 5: FizzBuzz (Function logic)
/// If n is divisible by 3 and 5, return "FizzBuzz"
/// If n is divisible by 3, return "Fizz"
/// If n is divisible by 5, return "Buzz"
/// Otherwise return the number as a String.
pub fn fizzbuzz(n: i32) -> String {
    todo!()
}

fn main() {
    println!("Run `rustc --test 03_functions.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(), ());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(5, 7), 12);
    }

    #[test]
    fn test_double_if_positive() {
        assert_eq!(double_if_positive(-5), 0);
        assert_eq!(double_if_positive(10), 20);
    }

    #[test]
    fn test_block_expression() {
        assert_eq!(block_expression(5), 20);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
