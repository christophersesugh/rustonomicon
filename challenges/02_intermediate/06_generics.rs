// Challenge: Generics
// Complete each function and method marked with `todo!()`.
// Verify with: rustc --test 06_generics.rs && ./06_generics

/// Exercise 1: Generic Function
/// In TypeScript: `function returnSame<T>(val: T): T`
pub fn return_same<T>(val: T) -> T {
    todo!()
}

/// Exercise 2: Generic Struct & Methods
/// In TypeScript: `interface Point<T> { x: T; y: T }`
#[derive(Debug, PartialEq, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Return a reference to the `x` field
    pub fn x(&self) -> &T {
        todo!()
    }

    /// Return a reference to the `y` field
    pub fn y(&self) -> &T {
        todo!()
    }
}

/// Specialized method only implemented for Point<f32>
impl Point<f32> {
    /// Calculate distance from origin (0.0, 0.0)
    pub fn distance_from_origin(&self) -> f32 {
        todo!()
    }
}

/// Exercise 3: Multiple Type Parameters & Method mixing
#[derive(Debug, PartialEq, Clone)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Mixes `self` with `other`: takes `first` from `self` and `second` from `other`
    pub fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        todo!()
    }
}

/// Exercise 4: Generics with Trait Bounds
/// Finds the largest element in a slice of items that can be ordered (`PartialOrd`).
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    todo!()
}

fn main() {
    println!("Run `rustc --test 06_generics.rs && ./06_generics` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_same() {
        assert_eq!(return_same(5), 5);
        assert_eq!(return_same("hello"), "hello");
    }

    #[test]
    fn test_point() {
        let p = Point { x: 5, y: 10 };
        assert_eq!(*p.x(), 5);
        assert_eq!(*p.y(), 10);
    }

    #[test]
    fn test_point_distance() {
        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(p.distance_from_origin(), 5.0);
    }
    
    #[test]
    fn test_pair_mixup() {
        let p1 = Pair { first: 5, second: 10.4 };
        let p2 = Pair { first: "Hello", second: 'c' };
        let p3 = p1.mixup(p2);
        assert_eq!(p3.first, 5);
        assert_eq!(p3.second, 'c');
    }

    #[test]
    fn test_largest() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&numbers), &100);
        
        let chars = vec!['y', 'm', 'a', 'q'];
        assert_eq!(largest(&chars), &'y');
    }
}
