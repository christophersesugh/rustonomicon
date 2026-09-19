# Project 5: Minigrep Extension

**Time:** 2–4 hours
**Practice:** `Result`, `?`, files, tests, library APIs, and environment variables

Use the repository's [`minigrep`](../../minigrep/) project. First make its basic search work from [Chapter 12](../../notes/12_io_project.md). Then improve it in small, test-driven steps.

## Required behavior

1. `cargo run -- query path/to/file.txt` prints matching lines.
2. A missing query or path produces a helpful error and non-zero exit status.
3. A missing file reports the I/O error; it must not panic.
4. `IGNORE_CASE=1 cargo run -- rust poem.txt` searches without case sensitivity.
5. Search functions live in `src/lib.rs` with unit tests.

## Milestones

1. Test case-sensitive search with a string literal.
2. Make the test pass without touching file I/O.
3. Parse `Config` from an argument iterator.
4. Read the file in `run` and print results in `main`.
5. Write a failing case-insensitive test, then implement it.

## Acceptance checks

- The tests do not rely on a particular working directory or the real poem file.
- `search` returns borrowed `&str` lines when it can, rather than cloning every line.
- User-facing errors are printed to standard error with `eprintln!`.
- `cargo test` passes before you manually try the CLI.

## Stretch goals

- Add line numbers.
- Support `--ignore-case` and `-i` flags.
- Return a custom error enum instead of `Box<dyn Error>` at the application boundary.

## Memory check

Why may a result `Vec<&str>` live only while the original file `String` lives? What would change if `search` returned `Vec<String>`?
