// Challenge: Iterators
// Complete each function and method marked with `todo!()`.
// Verify with: rustc --test 03_iterators.rs && ./03_iterators

/// Exercise 1: Iterator Basics
/// Given a vector of i32, return the sum of all elements using `.iter().sum()`.
pub fn sum_elements(v: &[i32]) -> i32 {
    todo!()
}

/// Exercise 2: Iterator Adapters (Map)
/// Given a vector of strings, use `.into_iter().map()` to append "!" to each string, and `.collect()` it into a new Vec<String>.
pub fn exclaim_all(v: Vec<String>) -> Vec<String> {
    todo!()
}

/// Exercise 3: Iterator Adapters (Filter)
/// Given a slice of i32, use `.iter().filter()` to keep only positive numbers (> 0), 
/// cloned into a new Vec<i32>.
pub fn only_positives(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Exercise 4: Chaining Adapters
/// Take a slice of strings, filter out empty strings, get their lengths, and sum them up.
pub fn sum_of_lengths(words: &[&str]) -> usize {
    todo!()
}

/// Exercise 5: Custom Iterator
/// Create a struct `Counter` that counts from 1 to 5.
pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Exercise 6: Iterator Combinators (Zip and Fold)
/// Takes two slices of numbers of equal length, computes the dot product:
/// (a[0]*b[0] + a[1]*b[1] + ... + a[n]*b[n]) using `.iter().zip(...)` and `.map().sum()` or `.fold()`.
pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    todo!()
}

fn main() {
    println!("Run `rustc --test 03_iterators.rs && ./03_iterators` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_elements() {
        assert_eq!(sum_elements(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_exclaim_all() {
        let v = vec![String::from("hello"), String::from("world")];
        assert_eq!(exclaim_all(v), vec!["hello!", "world!"]);
    }

    #[test]
    fn test_only_positives() {
        assert_eq!(only_positives(&[-1, 2, -3, 4]), vec![2, 4]);
    }

    #[test]
    fn test_sum_of_lengths() {
        assert_eq!(sum_of_lengths(&["hello", "", "world", ""]), 10);
    }

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_dot_product() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(dot_product(&a, &b), 32);
    }
}
