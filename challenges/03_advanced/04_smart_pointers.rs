// Challenge: Smart Pointers
// Complete each function and method marked with `todo!()`.
// Verify with: rustc --test 04_smart_pointers.rs && ./04_smart_pointers

use std::rc::Rc;
use std::cell::RefCell;

/// Exercise 1 & 2: Box<T> and Recursive Types
/// In Rust, recursive types must use an indirection pointer (like `Box`) because the compiler
/// needs to know the exact size of an enum at compile time.
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// Helper function to create a new boxed List node
pub fn cons(val: i32, tail: List) -> List {
    todo!()
}

/// Computes the sum of all elements in the boxed recursive List
pub fn sum_list(list: &List) -> i32 {
    todo!()
}

/// Exercise 3 & 4: Reference Counting (Rc<T>)
/// `Rc<T>` enables multiple owners for read-only data on the heap.
pub fn create_shared_node(val: String) -> (Rc<String>, Rc<String>) {
    // Return a tuple of two Rc pointers pointing to the SAME heap allocation
    todo!()
}

/// Exercise 5: RefCell<T> (Interior Mutability)
/// `RefCell<T>` moves borrow checking from compile-time to runtime.
/// Increment the inner value of the RefCell.
pub fn increment_refcell(cell: &RefCell<i32>) {
    todo!()
}

/// Exercise 6: Drop Trait
/// Create a struct `CleanUpTracker` that decrements an `Arc<Mutex<i32>>` or modifies a shared counter upon Drop.
pub struct Tracker {
    pub name: String,
    pub active_count: Rc<RefCell<i32>>,
}

impl Drop for Tracker {
    fn drop(&mut self) {
        todo!()
    }
}

fn main() {
    println!("Run `rustc --test 04_smart_pointers.rs && ./04_smart_pointers` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_list() {
        // List: 1 -> 2 -> 3 -> Nil
        let list = cons(1, cons(2, cons(3, List::Nil)));
        assert_eq!(sum_list(&list), 6);
    }

    #[test]
    fn test_rc_sharing() {
        let (rc1, rc2) = create_shared_node(String::from("shared"));
        assert_eq!(*rc1, "shared");
        assert_eq!(*rc2, "shared");
        // Strong count should be 2 because both point to the same memory
        assert_eq!(Rc::strong_count(&rc1), 2);
    }

    #[test]
    fn test_refcell_mutation() {
        let cell = RefCell::new(10);
        increment_refcell(&cell);
        assert_eq!(*cell.borrow(), 11);
    }

    #[test]
    fn test_drop_tracker() {
        let counter = Rc::new(RefCell::new(1));
        {
            let _tracker = Tracker {
                name: String::from("worker"),
                active_count: Rc::clone(&counter),
            };
            assert_eq!(*counter.borrow(), 1);
        } // _tracker goes out of scope and drops here
        assert_eq!(*counter.borrow(), 0);
    }
}
