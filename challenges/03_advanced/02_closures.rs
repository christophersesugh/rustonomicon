// Challenge: Closures
// Complete each function according to its description.

/// Exercise 1: Simple Closure
/// Write a function that returns a closure. The closure should take an `i32` and add 1 to it.
pub fn get_add_one() -> impl Fn(i32) -> i32 {
    |_x| todo!()
}

/// Exercise 2: Capturing Variables
/// Write a function that takes an `i32` (`offset`) and returns a closure that takes an `i32`
/// and adds the captured `offset` to it.
pub fn get_adder(offset: i32) -> impl Fn(i32) -> i32 {
    move |_x| todo!()
}

/// Exercise 3: FnMut
/// Write a function `apply_twice` that takes a mutable closure `f` of type `FnMut(i32) -> i32`
/// and a starting value `x`. It should apply the closure twice to `x` and return the result.
pub fn apply_twice<F>(mut f: F, x: i32) -> i32 
where 
    F: FnMut(i32) -> i32 
{
    todo!()
}

/// Exercise 4: FnOnce
/// Write a function `consume_and_return` that takes a closure `f` of type `FnOnce() -> String`.
/// It should call the closure and return its result.
pub fn consume_and_return<F>(f: F) -> String 
where 
    F: FnOnce() -> String 
{
    todo!()
}

/// Exercise 5: Using Closures with Iterators
/// Use iterator methods with closures to take a slice of integers, multiply each by 2, and collect into a Vec.
pub fn double_all(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("Run `rustc --test 02_closures.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_add_one() {
        let f = get_add_one();
        assert_eq!(f(5), 6);
    }

    #[test]
    fn test_get_adder() {
        let f = get_adder(10);
        assert_eq!(f(5), 15);
    }

    #[test]
    fn test_apply_twice() {
        let mut count = 0;
        let result = apply_twice(|x| {
            count += 1;
            x * 2
        }, 5);
        assert_eq!(result, 20);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_consume() {
        let s = String::from("consumed");
        let f = || s; // Captures and moves s
        assert_eq!(consume_and_return(f), "consumed");
    }

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
    }
}
