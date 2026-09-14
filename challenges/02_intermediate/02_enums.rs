// Challenge: Enums and Pattern Matching
// Complete each function according to its description.
// Verify with: rustc --test 02_enums.rs && ./02_enums

/// Exercise 1 & 2: Basic Enums and Exhaustive Match
/// In TypeScript: `type TrafficLight = 'Red' | 'Yellow' | 'Green'`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// Return "Stop" for Red, "Wait" for Yellow, and "Go" for Green.
pub fn light_message(light: TrafficLight) -> &'static str {
    todo!()
}

/// Exercise 3 & 4: Enums with Data (Discriminated Unions)
/// In TypeScript:
/// `type Message = { type: 'quit' } | { type: 'write'; text: string } | { type: 'move'; x: i32; y: i32 }`
#[derive(Debug, PartialEq, Clone)]
pub enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

/// Process a message:
/// - Quit -> "Quitting"
/// - Write(text) -> text
/// - Move { x, y } -> format "Moving to ({x}, {y})"
pub fn process_message(msg: Message) -> String {
    todo!()
}

/// Exercise 5: Option and Match
/// Given an `Option<i32>`, return its value multiplied by 2 if `Some`, or -1 if `None`.
pub fn double_or_negative_one(x: Option<i32>) -> i32 {
    todo!()
}

/// Exercise 6: `if let` Expression
/// Return true if the option contains an even number, false otherwise.
pub fn is_some_even(opt: Option<i32>) -> bool {
    todo!()
}

/// Exercise 7: `let else` Guard Clause
/// If `user_id` is `Some(id)`, return `format!("User_{id}")`. If `None`, return `String::from("Guest")`.
/// Implement this using `let Some(id) = user_id else { ... }`.
pub fn format_user_id(user_id: Option<u64>) -> String {
    todo!()
}

fn main() {
    println!("Run `rustc --test 02_enums.rs && ./02_enums` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        assert_eq!(light_message(TrafficLight::Red), "Stop");
        assert_eq!(light_message(TrafficLight::Yellow), "Wait");
        assert_eq!(light_message(TrafficLight::Green), "Go");
    }

    #[test]
    fn test_message() {
        assert_eq!(process_message(Message::Quit), "Quitting");
        assert_eq!(process_message(Message::Write(String::from("hello"))), "hello");
        assert_eq!(process_message(Message::Move { x: 10, y: 20 }), "Moving to (10, 20)");
    }

    #[test]
    fn test_double_or_negative_one() {
        assert_eq!(double_or_negative_one(Some(5)), 10);
        assert_eq!(double_or_negative_one(None), -1);
    }

    #[test]
    fn test_is_some_even() {
        assert_eq!(is_some_even(Some(4)), true);
        assert_eq!(is_some_even(Some(5)), false);
        assert_eq!(is_some_even(None), false);
    }

    #[test]
    fn test_format_user_id() {
        assert_eq!(format_user_id(Some(42)), "User_42");
        assert_eq!(format_user_id(None), "Guest");
    }
}
