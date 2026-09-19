# Project 1: CLI Greeter

**Time:** 30–45 minutes
**Practice:** variables, functions, `if`, `match`, `String`, and printing

Build a terminal program that asks for a name and an age, then prints a personal greeting. This is intentionally tiny: it lets you focus on the edit–check–run loop before ownership enters the picture.

## Required behavior

```text
What is your name? Amina
How old are you? 19
Hello, Amina! Next year you will be 20.
```

1. Read a name from standard input.
2. Remove the newline around the name.
3. Read an age and parse it as `u8`.
4. If parsing fails, print a friendly message instead of panicking.
5. Put the age calculation in a function: `fn age_next_year(age: u8) -> u8`.

## Milestones

1. Hard-code a name and make the greeting print.
2. Replace the hard-coded name with `stdin` input.
3. Add the age and parse it with `trim().parse::<u8>()`.
4. Use `match` to handle `Ok(age)` and `Err(_)`.
5. Write unit tests for `age_next_year`.

## Acceptance checks

- Empty input does not crash the program.
- `cargo test` checks the age calculation.
- `cargo clippy` has no warnings you do not understand.

## Stretch goals

- Greet differently before noon and after noon (pass the hour into a pure function; do not start with real clock APIs).
- Keep asking for an age until the user types a valid one.

## Memory check

Why does `read_line` need `&mut String`? Answer before reading Chapter 4: the input function temporarily changes the existing String buffer, but the caller still owns that buffer.
