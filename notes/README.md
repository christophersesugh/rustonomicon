# Notes

Chapter-by-chapter companion notes to [The Rust Programming Language](https://doc.rust-lang.org/book/).

## How These Notes Work

Each file corresponds to a chapter in the Rust Book. The notes are structured to:

1. **Start with a high-level explanation** — what the concept is and why it exists
2. **Draw TypeScript parallels** — connect new concepts to what you already know
3. **Dive into memory internals** — stack layouts, heap allocations, pointer diagrams
4. **List methods with internals** — for types with methods, every stable method is explained at both the API level and the implementation level
5. **Provide clear examples** — runnable Rust code with annotations

## Prerequisites

Start with [0_prerequisites.md](./0_prerequisites.md) if you need a refresher on:
- Computer memory layout (stack, heap, data segments)
- Pointers and memory addresses
- Memory management strategies (manual, GC, ownership)
- Bit/byte representation of data
- CPU registers and compilation

## Chapters

| File | Topic |
|------|-------|
| [0_prerequisites.md](./0_prerequisites.md) | Computer memory, stack, heap, pointers, compilation |
| [1_getting_started.md](./1_getting_started.md) | Installation, Hello World, Cargo |
| [2_programming_a_guessing_game.md](./2_programming_a_guessing_game.md) | First Rust program |
| [3_common_programming__concepts.md](./3_common_programming__concepts.md) | Variables, types, functions, control flow |
| [4_understanding_ownership.md](./4_understanding_ownership.md) | Ownership, borrowing, slices |
| [5_using_structs_to_structure_related_data.md](./5_using_structs_to_structure_related_data.md) | Structs, methods, associated functions |
| [6_enum_and_pattern_matching.md](./6_enum_and_pattern_matching.md) | Enums, Option, match, if let |
| [7_packages_crates_and_modules.md](./7_packages_crates_and_modules.md) | Crates, modules, paths, use |
| [8_common_collections.md](./8_common_collections.md) | Vec, String, HashMap |
| [9_error_handling.md](./9_error_handling.md) | panic!, Result, ? operator |
| [10_generic_types_traits_and_lifetimes.md](./10_generic_types_traits_and_lifetimes.md) | Generics, traits, lifetimes |
| [11_writing_automated_tests.md](./11_writing_automated_tests.md) | Writing and running tests |
| [12_io_project.md](./12_io_project.md) | minigrep CLI project |
| [13_closures_and_iterators.md](./13_closures_and_iterators.md) | Closures, iterators |
| [14_more_about_cargo_and_crates.md](./14_more_about_cargo_and_crates.md) | Publishing, workspaces |
| [15_smart_pointers.md](./15_smart_pointers.md) | Box, Rc, RefCell |
| [16_fearless_concurrency.md](./16_fearless_concurrency.md) | Threads, channels, Mutex |
| [17_asynchronous_programming.md](./17_asynchronous_programming.md) | async/await, futures |
| [18_object_oriented_programming_features.md](./18_object_oriented_programming_features.md) | Trait objects, OOP patterns |
| [19_patterns_and_matching.md](./19_patterns_and_matching.md) | Pattern matching, destructuring |
| [20_advanced_features.md](./20_advanced_features.md) | Unsafe, advanced traits, macros |
| [21_multithreaded_web_server.md](./21_multithreaded_web_server.md) | Final project |
| [22_appendix.md](./22_appendix.md) | Keywords, operators, tools |

## Diagrams

Visual diagrams are stored in [diagrams/](./diagrams/) and referenced throughout the notes.
