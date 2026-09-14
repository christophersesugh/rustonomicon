// Challenge: Advanced Pattern Matching
// Complete each function according to its description.

/// Exercise 1: Matching Multiple Patterns
/// Write a function that takes an `i32` and returns a static string slice.
/// If 1, 2, or 3, return "Small".
/// If 4 through 10 (inclusive), return "Medium".
/// Anything else, return "Large".
pub fn match_size(n: i32) -> &'static str {
    todo!()
}

/// Exercise 2: Destructuring Structs
/// Given a struct `Point { x: i32, y: i32 }`, use a match expression to return:
/// "On the x axis at {x}" if y is 0
/// "On the y axis at {y}" if x is 0
/// "On neither axis" otherwise
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn match_point(p: Point) -> String {
    todo!()
}

/// Exercise 3: Match Guards
/// Take an `Option<i32>`. Use match with a guard (`if`) to return:
/// "Got an even number: {n}" if it's Some containing an even number.
/// "Got an odd number: {n}" if it's Some containing an odd number.
/// "No number" if it's None.
pub fn match_guard(opt: Option<i32>) -> String {
    todo!()
}

/// Exercise 4: @ Bindings
/// Given an enum `Message { Hello { id: i32 } }`, use a match and `@` binding to:
/// - Return "Found an id in range: {id}" if id is between 3 and 7 (inclusive).
/// - Return "Found some other id: {id}" for any other id.
pub enum Message {
    Hello { id: i32 },
}

pub fn match_binding(msg: Message) -> String {
    todo!()
}

fn main() {
    println!("Run `rustc --test 06_pattern_matching.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_size() {
        assert_eq!(match_size(2), "Small");
        assert_eq!(match_size(7), "Medium");
        assert_eq!(match_size(42), "Large");
    }

    #[test]
    fn test_match_point() {
        assert_eq!(match_point(Point { x: 5, y: 0 }), "On the x axis at 5");
        assert_eq!(match_point(Point { x: 0, y: 10 }), "On the y axis at 10");
        assert_eq!(match_point(Point { x: 3, y: 3 }), "On neither axis");
    }

    #[test]
    fn test_match_guard() {
        assert_eq!(match_guard(Some(4)), "Got an even number: 4");
        assert_eq!(match_guard(Some(5)), "Got an odd number: 5");
        assert_eq!(match_guard(None), "No number");
    }

    #[test]
    fn test_match_binding() {
        assert_eq!(match_binding(Message::Hello { id: 5 }), "Found an id in range: 5");
        assert_eq!(match_binding(Message::Hello { id: 10 }), "Found some other id: 10");
    }
}
