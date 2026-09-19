# 12. I/O Project: Build `minigrep`

`minigrep` is a small command-line search tool. You run it with a search query and a file path; it prints lines containing that query. It is a useful project because it joins several real-world ideas without adding much new syntax.

```bash
cargo run -- duct poem.txt
```

The program should:

1. Read command-line arguments.
2. Turn them into a `Config` value.
3. Read the chosen file.
4. Search its text.
5. Print matching lines or explain an error.

## Keep `main` small

Use a binary crate for the command-line boundary and a library crate for testable logic:

```text
minigrep/
├── Cargo.toml
└── src/
    ├── lib.rs     # search functions and unit tests
    └── main.rs    # reads args, calls the library, prints errors
```

This is not ceremony. `main` touches the outside world: arguments, files, terminal output, and process exit. `lib.rs` can contain pure functions, which are much easier to test.

## Model arguments as a type

```rust
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // skip the executable name

        let query = args.next().ok_or("Missing search query")?;
        let file_path = args.next().ok_or("Missing file path")?;

        Ok(Self { query, file_path, ignore_case: false })
    }
}
```

`env::args()` yields owned `String` values one by one. `Config::build` takes ownership of the two values it needs. That is a good choice: the configuration needs to outlive the argument iterator.

`ok_or(...)` turns `Option<String>` into `Result<String, &str>`, and `?` returns the error early when an argument is missing. Chapter 9 explains that pattern in detail.

## Search without taking ownership of the file

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

This function borrows the query and file contents. Each result is a slice pointing into `contents`; no matching line is copied.

```text
contents owns one String allocation on the heap
┌──────────────────────────────────────┐
│ fast, safe, productive\\n...          │
└──────────────────────────────────────┘
       ▲                 ▲
       │                 │
Vec holds borrowed line slices (pointer + length each)
```

The `'a` annotation connects result lines to `contents`, so callers cannot keep the results after dropping the file text.

## Handle I/O errors at the edge

```rust
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

`read_to_string` may fail because the path is wrong, a file is not readable, or its bytes are not valid UTF-8. `?` sends the error back to `main`, where the program can print it and exit with a non-zero status.

`Box<dyn Error>` is acceptable at an application boundary because several different error types may reach that one place. Inside a library, prefer a more precise error type whenever its callers need to respond differently.

## Test the search behavior

```rust
#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn finds_matching_lines() {
        let text = "safe, fast, productive.\\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search("duct", text));
    }
}
```

This test does not read a file. It gives `search` exact input and checks exact output. Fast, deterministic tests like this are the foundation of confidence.

## Build it in this repository

The starter project is in [`minigrep/`](../minigrep/). Run its tests with:

```bash
cd minigrep
cargo test
```

Then add case-insensitive search using an `IGNORE_CASE` environment variable or a flag. Write the failing test first.
