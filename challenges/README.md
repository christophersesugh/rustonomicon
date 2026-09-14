# Rustonomicon Challenges

Welcome to the Rustonomicon challenges! This repository provides practical, hands-on exercises to help you master Rust, especially if you're coming from a TypeScript background.

## Structure

The challenges are organized by difficulty and topic:

- **01_beginner**: Core concepts like variables, ownership, references, and slices.
- **02_intermediate**: Enums, structs, traits, generics, error handling, collections, and testing.
- **03_advanced**: Lifetimes, closures, iterators, smart pointers, concurrency, and advanced pattern matching.

## How to Complete a Challenge

Each file contains several exercises marked with `todo!()`. Your goal is to replace `todo!()` with the correct implementation to make the code compile and tests pass.

### Running Tests

You can run the tests for a specific file using standard `rustc` and running the test binary, or if you've added them to a Cargo workspace, using Cargo.

For example, to run the tests in `01_variables.rs`:
```bash
rustc --test 01_variables.rs
./01_variables
```

Happy Rusty coding!
