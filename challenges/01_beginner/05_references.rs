// Challenge: References and Borrowing
// Complete each function according to its description.

/// Exercise 1: Immutable Reference
/// Take a reference to a String and return its length.
/// This does not take ownership!
pub fn get_length(s: &String) -> usize {
    todo!()
}

/// Exercise 2: Mutable Reference
/// Take a mutable reference to a String and append " world" to it.
/// Return nothing.
pub fn append_world(s: &mut String) {
    todo!()
}

/// Exercise 3: Dereferencing
/// Take a mutable reference to an i32 and increment its value by 1.
pub fn increment(n: &mut i32) {
    todo!()
}

/// Exercise 4: Multiple Immutable Borrows
/// Take two string references and return the length of the longer one.
pub fn longest_length(s1: &String, s2: &String) -> usize {
    todo!()
}

/// Exercise 5: Borrowing in a tuple
/// Given a tuple `(String, String)`, return a tuple of references to its elements.
pub fn borrow_elements(t: &(String, String)) -> (&String, &String) {
    todo!()
}

fn main() {
    println!("Run `rustc --test 05_references.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length() {
        let s = String::from("hello");
        assert_eq!(get_length(&s), 5);
        // Ensure s is still usable!
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_increment() {
        let mut n = 5;
        increment(&mut n);
        assert_eq!(n, 6);
    }

    #[test]
    fn test_longest_length() {
        let s1 = String::from("rust");
        let s2 = String::from("programming");
        assert_eq!(longest_length(&s1, &s2), 11);
    }

    #[test]
    fn test_borrow_elements() {
        let t = (String::from("a"), String::from("b"));
        let (a, b) = borrow_elements(&t);
        assert_eq!(a, "a");
        assert_eq!(b, "b");
    }
}
