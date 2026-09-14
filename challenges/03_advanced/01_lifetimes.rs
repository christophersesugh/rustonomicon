// Challenge: Lifetimes
// Complete each function and method marked with `todo!()`.
// Verify with: rustc --test 01_lifetimes.rs && ./01_lifetimes

/// Exercise 1: Lifetime Annotations in Functions
/// Takes two string slices with lifetime `'a` and returns whichever is longer.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!()
}

/// Exercise 2 & 3: Struct Lifetimes and Methods
/// A struct that holds a reference must have an explicit lifetime annotation.
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// Lifetime elision rule applies here: &self gives its lifetime to output if any,
    /// but returning an integer doesn't need lifetime annotations at all!
    pub fn level(&self) -> i32 {
        todo!()
    }

    /// Announces and returns the excerpt's part
    pub fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        todo!()
    }
}

/// Exercise 4: Static Lifetime
/// Returns a string slice with a `'static` lifetime (lives for the entire program execution).
pub fn get_static_string() -> &'static str {
    todo!()
}

/// Exercise 5: Multiple Lifetimes
/// Returns whichever string slice matches the boolean flag.
/// Both slices must live at least as long as the returned reference `'a`.
pub fn first_or_second<'a>(x: &'a str, y: &'a str, return_first: bool) -> &'a str {
    todo!()
}

fn main() {
    println!("Run `rustc --test 01_lifetimes.rs && ./01_lifetimes` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = String::from("abcd");
        let s2 = "xyz";
        assert_eq!(longest(s1.as_str(), s2), "abcd");

        let s3 = "hello world";
        let s4 = "rust";
        assert_eq!(longest(s3, s4), "hello world");
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = text.split('.').next().unwrap();
        let i = ImportantExcerpt { part: first_sentence };
        assert_eq!(i.part, "Call me Ishmael");
        assert_eq!(i.level(), 3);
        assert_eq!(i.announce_and_return_part("Important!"), "Call me Ishmael");
    }

    #[test]
    fn test_static_string() {
        let s: &'static str = get_static_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_first_or_second() {
        let a = "apple";
        let b = "banana";
        assert_eq!(first_or_second(a, b, true), "apple");
        assert_eq!(first_or_second(a, b, false), "banana");
    }
}
