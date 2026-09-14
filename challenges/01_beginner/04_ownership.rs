// Challenge: Ownership
// Complete each function according to its description.

/// Exercise 1: Move Semantics
/// Given a String, return it to transfer ownership back to the caller.
pub fn take_and_return(s: String) -> String {
    todo!()
}

/// Exercise 2: Clone
/// Given a String, return a tuple containing TWO copies of the String.
/// You'll need to clone it.
pub fn duplicate_string(s: String) -> (String, String) {
    todo!()
}

/// Exercise 3: Copy Trait
/// Integers implement Copy. Take an i32, and return a tuple with two copies.
/// Notice you don't need `.clone()`!
pub fn duplicate_int(n: i32) -> (i32, i32) {
    todo!()
}

/// Exercise 4: Ownership in Tuples
/// Given a tuple `(String, i32)`, extract and return just the String.
/// This will partially move the tuple.
pub fn extract_string(t: (String, i32)) -> String {
    todo!()
}

/// Exercise 5: String Length and Ownership
/// Take a String, calculate its length, and return BOTH the original String and its length.
pub fn get_length_with_string(s: String) -> (String, usize) {
    todo!()
}

fn main() {
    println!("Run `rustc --test 04_ownership.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_and_return() {
        let s1 = String::from("hello");
        let s2 = take_and_return(s1);
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_duplicate_string() {
        let s = String::from("rust");
        let (s1, s2) = duplicate_string(s);
        assert_eq!(s1, "rust");
        assert_eq!(s2, "rust");
    }

    #[test]
    fn test_duplicate_int() {
        let (n1, n2) = duplicate_int(42);
        assert_eq!(n1, 42);
        assert_eq!(n2, 42);
    }

    #[test]
    fn test_extract_string() {
        let t = (String::from("data"), 100);
        assert_eq!(extract_string(t), "data");
    }

    #[test]
    fn test_get_length() {
        let (s, len) = get_length_with_string(String::from("rust"));
        assert_eq!(s, "rust");
        assert_eq!(len, 4);
    }
}
