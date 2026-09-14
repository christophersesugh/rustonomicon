// Challenge: Structs
// Complete each function and method marked with `todo!()`.
// Verify by running: rustc --test 01_structs.rs && ./01_structs

/// Exercise 1: Struct Definition and Instantiation
/// In TypeScript: `interface User { username: string; email: string; active: bool; sign_in_count: number }`
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u64,
}

/// Instantiate a new `User` with the provided parameters, setting `active: true` and `sign_in_count: 1`.
pub fn build_user(email: String, username: String) -> User {
    todo!()
}

/// Exercise 2: Struct Update Syntax
/// Return a new `User` with updated `email`, preserving `username`, `active`, and `sign_in_count` from `original`.
pub fn update_user_email(original: User, new_email: String) -> User {
    todo!()
}

/// Exercise 3: Tuple Structs
/// Tuple structs give semantic meaning to a tuple without naming each individual field.
/// Color represents (R, G, B) as u8 values.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

pub fn invert_color(color: Color) -> Color {
    todo!()
}

/// Exercise 4 & 5: Struct Methods & Associated Constructors
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Associated constructor function (like a static method `Rectangle.square(size)` in TS)
    pub fn square(size: u32) -> Self {
        todo!()
    }

    /// Method borrowing `&self`: calculates the rectangle area
    pub fn area(&self) -> u32 {
        todo!()
    }

    /// Method borrowing `&self`: returns true if self can hold another rectangle completely
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// Method taking `&mut self`: scales both width and height by `factor`
    pub fn scale(&mut self, factor: u32) {
        todo!()
    }
}

fn main() {
    println!("Run `rustc --test 01_structs.rs && ./01_structs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user() {
        let user = build_user(String::from("dev@rust.org"), String::from("ferris"));
        assert_eq!(user.username, "ferris");
        assert_eq!(user.email, "dev@rust.org");
        assert!(user.active);
        assert_eq!(user.sign_in_count, 1);
    }

    #[test]
    fn test_update_user_email() {
        let u1 = User {
            username: String::from("alice"),
            email: String::from("alice@old.com"),
            active: true,
            sign_in_count: 42,
        };
        let u2 = update_user_email(u1, String::from("alice@new.com"));
        assert_eq!(u2.username, "alice");
        assert_eq!(u2.email, "alice@new.com");
        assert_eq!(u2.active, true);
        assert_eq!(u2.sign_in_count, 42);
    }

    #[test]
    fn test_color_inversion() {
        let red = Color(255, 0, 100);
        let inverted = invert_color(red);
        assert_eq!(inverted, Color(0, 255, 155));
    }

    #[test]
    fn test_rectangle_methods() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        assert_eq!(rect1.area(), 1500);
        assert!(rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
    }

    #[test]
    fn test_rectangle_square_and_scale() {
        let mut sq = Rectangle::square(10);
        assert_eq!(sq.width, 10);
        assert_eq!(sq.height, 10);
        assert_eq!(sq.area(), 100);

        sq.scale(3);
        assert_eq!(sq.width, 30);
        assert_eq!(sq.height, 30);
        assert_eq!(sq.area(), 900);
    }
}
