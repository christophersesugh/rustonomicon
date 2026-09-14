# Rustonomicon

A deep-dive companion to [The Rust Programming Language](https://doc.rust-lang.org/book/) book — written for developers with a TypeScript background who want to truly understand Rust, from high-level concepts down to stack frames, heap allocations, and memory layouts.

## Who Is This For?

You should use this resource if:

- You have a solid background in **TypeScript** (or JavaScript)
- You're reading or have read [The Rust Book](https://doc.rust-lang.org/book/)
- You want to understand **how things work internally** — not just the syntax
- You want to know what happens in memory when you write `let s = String::from("hello")`

## What's Inside

### 📖 [Notes](./notes/)

Chapter-by-chapter notes that mirror the Rust Book structure. Each chapter starts with high-level explanations and then dives into:

- **Memory internals** — stack layouts, heap allocations, pointer diagrams
- **Behind the scenes** — how the compiler implements ownership, borrowing, vtables, etc.
- **TypeScript comparisons** — bridging what you already know to Rust's model
- **Method reference** — for types with methods, all stable methods are listed with high-level and internal explanations

Start with [Prerequisites](./notes/0_prerequisites.md) if you need a refresher on computer memory (stack, heap, pointers, CPU registers).

### 🏋️ [Challenges](./challenges/)

Hands-on Rust exercises ranging from beginner to advanced:

- **Beginner** — variables, types, functions, ownership, references, slices
- **Intermediate** — structs, enums, collections, error handling, traits, generics, testing
- **Advanced** — lifetimes, closures, iterators, smart pointers, concurrency, pattern matching

Each challenge file contains 5-8 exercises with `todo!()` bodies, clear instructions, and test cases. Complete them all and you'll be confident writing Rust code.

## Chapter Index

| # | Chapter | Topic |
|---|---------|-------|
| 0 | [Prerequisites](./notes/0_prerequisites.md) | Memory, stack, heap, pointers, compilation |
| 1 | [Getting Started](./notes/1_getting_started.md) | Installation, Hello World, Cargo |
| 2 | [Guessing Game](./notes/2_programming_a_guessing_game.md) | First Rust program |
| 3 | [Common Concepts](./notes/3_common_programming__concepts.md) | Variables, types, functions, control flow |
| 4 | [Ownership](./notes/4_understanding_ownership.md) | Ownership, borrowing, slices |
| 5 | [Structs](./notes/5_using_structs_to_structure_related_data.md) | Structs, methods, associated functions |
| 6 | [Enums & Matching](./notes/6_enum_and_pattern_matching.md) | Enums, Option, match, if let |
| 7 | [Packages & Modules](./notes/7_packages_crates_and_modules.md) | Crates, modules, paths, use |
| 8 | [Collections](./notes/8_common_collections.md) | Vec, String, HashMap |
| 9 | [Error Handling](./notes/9_error_handling.md) | panic!, Result, ? operator |
| 10 | [Generics & Traits](./notes/10_generic_types_traits_and_lifetimes.md) | Generics, traits, lifetimes |
| 11 | [Testing](./notes/11_writing_automated_tests.md) | Writing and running tests |
| 12 | [I/O Project](./notes/12_io_project.md) | minigrep CLI project |
| 13 | [Closures & Iterators](./notes/13_closures_and_iterators.md) | Closures, iterators |
| 14 | [Cargo & Crates.io](./notes/14_more_about_cargo_and_crates.md) | Publishing, workspaces |
| 15 | [Smart Pointers](./notes/15_smart_pointers.md) | Box, Rc, RefCell |
| 16 | [Concurrency](./notes/16_fearless_concurrency.md) | Threads, message passing, shared state |
| 17 | [Async Programming](./notes/17_asynchronous_programming.md) | async/await, futures |
| 18 | [OOP Features](./notes/18_object_oriented_programming_features.md) | Trait objects, OOP patterns |
| 19 | [Patterns](./notes/19_patterns_and_matching.md) | Pattern matching, destructuring |
| 20 | [Advanced Features](./notes/20_advanced_features.md) | Unsafe, advanced traits, macros |
| 21 | [Web Server](./notes/21_multithreaded_web_server.md) | Final project |
| 22 | [Appendix](./notes/22_appendix.md) | Keywords, operators, tools |

## How to Use

1. **Read the Rust Book chapter first** — get the high-level concepts
2. **Read the corresponding notes** — understand the internals and memory model
3. **Complete the challenges** — practice writing Rust code
4. **Repeat** — come back to the notes as reference when writing real Rust

## Running Challenges

```bash
# Navigate to a challenge
cd challenges/01_beginner

# Run a specific challenge
rustc 01_variables.rs && ./01_variables

# Or run tests
rustc --test 01_variables.rs && ./01_variables
```