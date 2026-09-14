// Challenge: Variables and Mutability
// Complete each function according to its description.

/// Exercise 1: Create and return an immutable variable
/// Return the value 42
pub fn exercise_1() -> i32 {
    // let x = 42;
    // x
    todo!()
}

/// Exercise 2: Mutability
/// Take a mutable variable `x` that starts at 10, add 5 to it, and return it.
pub fn exercise_2() -> i32 {
    // let mut x = 10;
    // x += 5;
    // x
    todo!()
}

/// Exercise 3: Shadowing
/// Declare a variable `x` with value 5. Shadow it with a new `x` that multiplies the previous by 2, and return it.
pub fn exercise_3() -> i32 {
    todo!()
}

/// Exercise 4: Constants
/// Define a constant `MAX_SPEED` of type u32 set to 120 and return it.
pub fn exercise_4() -> u32 {
    todo!()
}

/// Exercise 5: Scope and Shadowing
/// Declare `x` as 10. Open a block scope where you shadow `x` to be 20. 
/// Return the `x` from the outer scope after the block ends.
pub fn exercise_5() -> i32 {
    todo!()
}

fn main() {
    println!("Run `rustc --test 01_variables.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1() {
        assert_eq!(exercise_1(), 42);
    }

    #[test]
    fn test_exercise_2() {
        assert_eq!(exercise_2(), 15);
    }

    #[test]
    fn test_exercise_3() {
        assert_eq!(exercise_3(), 10);
    }

    #[test]
    fn test_exercise_4() {
        assert_eq!(exercise_4(), 120);
    }

    #[test]
    fn test_exercise_5() {
        assert_eq!(exercise_5(), 10);
    }
}
