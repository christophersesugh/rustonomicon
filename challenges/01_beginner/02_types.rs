// Challenge: Primitive Types and Annotations
// Complete each function according to its description.

/// Exercise 1: Booleans
/// Return a boolean value of true.
pub fn exercise_1() -> bool {
    todo!()
}

/// Exercise 2: Characters
/// Return the character '🦀'.
pub fn exercise_2() -> char {
    todo!()
}

/// Exercise 3: Type Annotations
/// Create a float variable with an explicit type annotation of f32 set to 3.14. Return it.
pub fn exercise_3() -> f32 {
    todo!()
}

/// Exercise 4: Tuples
/// Return a tuple containing an i32 (10), a f64 (20.5), and a char ('a').
pub fn exercise_4() -> (i32, f64, char) {
    todo!()
}

/// Exercise 5: Arrays
/// Return an array of 5 zeros of type u8.
pub fn exercise_5() -> [u8; 5] {
    todo!()
}

/// Exercise 6: Casting
/// Take the f64 45.67, cast it to an i32, and return it.
pub fn exercise_6() -> i32 {
    todo!()
}

fn main() {
    println!("Run `rustc --test 02_types.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1() {
        assert_eq!(exercise_1(), true);
    }

    #[test]
    fn test_exercise_2() {
        assert_eq!(exercise_2(), '🦀');
    }

    #[test]
    fn test_exercise_3() {
        assert_eq!(exercise_3(), 3.14);
    }

    #[test]
    fn test_exercise_4() {
        assert_eq!(exercise_4(), (10, 20.5, 'a'));
    }

    #[test]
    fn test_exercise_5() {
        assert_eq!(exercise_5(), [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_exercise_6() {
        assert_eq!(exercise_6(), 45);
    }
}
