# Build Projects

These are deliberately small. Finish the required version before adding a stretch goal. A completed small program teaches more than a half-finished ambitious app.

| # | Project | Main practice | Prerequisites |
| --- | --- | --- | --- |
| 1 | [CLI Greeter](./01_cli_greeter.md) | variables, functions, control flow | Chapters 1–3 |
| 2 | [Guessing Game](./02_guessing_game.md) | input, loops, parsing, `Result` | Chapters 1–3 |
| 3 | [Task Tracker](./03_task_tracker.md) | structs, enums, ownership, modules | Chapters 4–7 |
| 4 | [Word Frequency](./04_word_frequency.md) | `String`, slices, `HashMap`, iterators | Chapters 8 and 13 |
| 5 | [Minigrep Extension](./05_minigrep_extension.md) | errors, tests, files, public APIs | Chapters 9–12 |
| 6 | [Parallel Log Analyzer](./06_parallel_log_analyzer.md) | closures, channels, `Arc`, `Mutex` | Chapters 13, 15, and 16 |
| 7 | [Tiny Web Server](./07_tiny_web_server.md) | TCP, thread pools, `Drop` | Chapters 16 and 21 |

## How to build a project

1. Make a new folder outside `challenges/`, then run `cargo new project_name`.
2. Copy only the requirements into your own TODO list; do not copy a solution from a previous project.
3. Work one milestone at a time. Run `cargo check` after a small change.
4. Add at least one test for every behavior that does not need terminal input or a real file.
5. Finish with `cargo fmt`, `cargo clippy`, and `cargo test`.

## When you are stuck

Write down three facts: the input type, the output type, and who should own the data. Then create the smallest function that connects them. The compiler is especially helpful once your function signatures are honest.

Avoid adding a dependency until the standard library version works. Every dependency is useful knowledge later; it should not hide the Rust concept you are practising now.
