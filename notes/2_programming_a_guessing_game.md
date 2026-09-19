# 2. Build a Guessing Game

This chapter combines the basics into one small program. A computer picks a number; the player types guesses until they get it right. The game teaches input, output, parsing text into a number, comparisons, loops, and errors.

Do not try to understand every line on the first pass. First see the program work; then trace one turn of the loop.

## Start a project and add one library

```bash
cargo new guessing_game
cd guessing_game
cargo add rand
```

`rand` is a **crate**: reusable Rust code published as a package. `cargo add` records the dependency in `Cargo.toml` and Cargo downloads a compatible version.

## A complete first version

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    println!("Guess a number from 1 to 100.");

    loop {
        println!("Please enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input");

        let guess: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a whole number.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## One turn, slowly

`let mut input = String::new();` creates an empty, growable string. `mut` permits the next line to change it.

`read_line(&mut input)` asks standard input to append the characters the player typed. The `&mut` says, “borrow this String temporarily and change it; do not take it away.” The player usually presses Enter, so `input` includes a trailing newline.

`input.trim()` borrows a view of the same text with surrounding whitespace removed. `.parse()` tries to turn that text into a number. It can fail when the player enters `hello`, so it returns `Result`:

```text
valid text ──parse──> Ok(number)
invalid text ─parse──> Err(problem)
```

`match` handles both possibilities. `continue` skips straight to the next loop turn. `break` leaves the loop completely.

## The data in memory

On each loop turn, the stack contains small bookkeeping values such as `guess` and a `String` handle. The text bytes typed by the user live in a growable heap allocation owned by `input`.

```text
stack frame                         heap
┌────────────────────┐              ┌─────────────────┐
│ secret: u32        │              │ "42\\n"          │
│ input: ptr,len,cap │─────────────▶│ bytes from stdin │
│ guess: u32         │              └─────────────────┘
└────────────────────┘
```

At the end of that loop turn, `input` goes out of scope. Rust drops it and frees its heap buffer. The next turn creates a fresh empty `String`.

## TypeScript bridge

The rough JavaScript shape is `Number(input.trim())`, but JavaScript can return `NaN`. Rust instead makes the failure visible in the type: `parse::<u32>()` produces `Result<u32, _>`. You must decide what an invalid answer means.

## Small upgrades to build yourself

1. Reject guesses outside `1..=100` without ending the game.
2. Count attempts and show the count when the player wins.
3. Let the player choose the upper bound.
4. Add a `q` command that exits cleanly.

For a fuller project brief with acceptance checks, see [Guessing Game](../challenges/04_projects/02_guessing_game.md).
