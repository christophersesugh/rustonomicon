// Challenge: Concurrency
// Complete each function according to its description.

use std::thread;
use std::sync::{mpsc, Mutex, Arc};

/// Exercise 1: Spawning a Thread
/// Spawn a thread that returns the value 42. Wait for it to finish and return its result.
pub fn spawn_and_join() -> i32 {
    todo!()
}

/// Exercise 2: Message Passing (Channels)
/// Create a channel. Spawn a thread that sends the string "hello" to the channel.
/// Receive the string in the main thread and return it.
pub fn send_receive() -> String {
    todo!()
}

/// Exercise 3: Shared State (Mutex)
/// Create a `Mutex<i32>` initialized to 0. Lock it, add 5 to the value, unlock (drop the lock), 
/// and then extract and return the inner i32.
pub fn mutex_add() -> i32 {
    todo!()
}

/// Exercise 4: Arc and Mutex (Multi-threaded shared state)
/// Take an `Arc<Mutex<i32>>` (which acts as a shared counter). 
/// Spawn 5 threads, where each thread increments the counter by 1.
/// Wait for all threads to finish.
pub fn threaded_counter(counter: Arc<Mutex<i32>>) {
    todo!()
}

fn main() {
    println!("Run `rustc --test 05_concurrency.rs` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_join() {
        assert_eq!(spawn_and_join(), 42);
    }

    #[test]
    fn test_send_receive() {
        assert_eq!(send_receive(), "hello");
    }

    #[test]
    fn test_mutex_add() {
        assert_eq!(mutex_add(), 5);
    }

    #[test]
    fn test_threaded_counter() {
        let counter = Arc::new(Mutex::new(0));
        threaded_counter(Arc::clone(&counter));
        assert_eq!(*counter.lock().unwrap(), 5);
    }
}
