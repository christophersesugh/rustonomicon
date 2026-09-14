// Challenge: Slices
// Complete each function according to its description.

/// Exercise 1: String Slices
/// Return a slice representing the first 5 characters of the given string reference.
pub fn first_five_chars(s: &str) -> &str {
    todo!()
}

/// Exercise 2: Array Slices
/// Take an array slice `&[i32]` and return a slice of all elements except the first and last.
/// You can assume the slice has at least 2 elements.
pub fn middle_elements(arr: &[i32]) -> &[i32] {
    todo!()
}

/// Exercise 3: Find Word
/// Find the first word in a string (words separated by spaces) and return it as a slice.
/// If there are no spaces, return the whole string.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Exercise 4: Slicing String literals
/// Create a function that takes a string literal (which is already a slice `&str`)
/// and returns the length of the string slice.
pub fn literal_length(s: &str) -> usize {
    todo!()
}

/// Exercise 5: Mutable Array Slices
/// Take a mutable slice `&mut [i32]` and set the first element to 0.
/// Assume the slice has at least 1 element.
pub fn zero_first(arr: &mut [i32]) {
    todo!()
}

fn main() {
    println!("Run `rustc --test 06_slices.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five_chars() {
        assert_eq!(first_five_chars("hello world"), "hello");
    }

    #[test]
    fn test_middle_elements() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(middle_elements(&arr), &[2, 3, 4]);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("rust is awesome"), "rust");
        assert_eq!(first_word("typescript"), "typescript");
    }

    #[test]
    fn test_literal_length() {
        assert_eq!(literal_length("hello"), 5);
    }

    #[test]
    fn test_zero_first() {
        let mut arr = [1, 2, 3];
        zero_first(&mut arr);
        assert_eq!(arr, [0, 2, 3]);
    }
}
