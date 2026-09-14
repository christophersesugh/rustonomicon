// Challenge: Collections
// Complete each function according to its description.

use std::collections::HashMap;

/// Exercise 1: Vec Creation
/// Create and return a vector containing the numbers 1, 2, 3, 4, 5.
pub fn create_vec() -> Vec<i32> {
    todo!()
}

/// Exercise 2: Vec Modification
/// Take a mutable vector of integers and push the number 42 to the end.
pub fn push_42(v: &mut Vec<i32>) {
    todo!()
}

/// Exercise 3: Vec Filtering
/// Given a vector of integers, return a new vector containing only the even numbers.
pub fn filter_evens(v: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Exercise 4: String Operations
/// Given a slice of words `&[&str]`, concatenate them into a single `String` separated by spaces.
pub fn join_words(words: &[&str]) -> String {
    todo!()
}

/// Exercise 5: HashMap Creation
/// Create and return a HashMap mapping the `String` "Alice" to 10 and "Bob" to 20.
pub fn create_scores() -> HashMap<String, i32> {
    todo!()
}

/// Exercise 6: HashMap Update
/// Given a mutable HashMap, if the key "Charlie" exists, increment its value by 1.
/// If it doesn't exist, insert it with a value of 1.
pub fn update_score(scores: &mut HashMap<String, i32>) {
    todo!()
}

fn main() {
    println!("Run `rustc --test 03_collections.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vec() {
        assert_eq!(create_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_push_42() {
        let mut v = vec![1, 2, 3];
        push_42(&mut v);
        assert_eq!(v, vec![1, 2, 3, 42]);
    }

    #[test]
    fn test_filter_evens() {
        assert_eq!(filter_evens(vec![1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_join_words() {
        assert_eq!(join_words(&["hello", "rust", "world"]), "hello rust world");
    }

    #[test]
    fn test_create_scores() {
        let scores = create_scores();
        assert_eq!(scores.get("Alice"), Some(&10));
        assert_eq!(scores.get("Bob"), Some(&20));
    }

    #[test]
    fn test_update_score() {
        let mut scores = HashMap::new();
        update_score(&mut scores);
        assert_eq!(scores.get("Charlie"), Some(&1));
        
        update_score(&mut scores);
        assert_eq!(scores.get("Charlie"), Some(&2));
    }
}
