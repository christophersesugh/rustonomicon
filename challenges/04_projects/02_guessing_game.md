# Project 2: Guessing Game

**Time:** 45–90 minutes
**Practice:** loops, comparisons, input, parsing, `Result`, and a dependency

Build the number-guessing game from [Chapter 2](../../notes/2_programming_a_guessing_game.md) once without looking at the completed example. The point is recall, not cleverness.

## Setup

```bash
cargo new guessing_game
cd guessing_game
cargo add rand
```

## Required behavior

1. Generate a secret `u32` from 1 through 100.
2. Repeatedly ask the player for a guess.
3. Say whether it is too high, too low, or correct.
4. Invalid text must show a helpful message and ask again.
5. Winning must leave the loop with `break`.

## Milestones

1. Write a function `compare_guess(guess: u32, secret: u32) -> &'static str` and test all three outcomes.
2. Generate the random secret and call that function with a hard-coded guess.
3. Read one guess from the terminal.
4. Put the turn inside `loop`.
5. Handle parse failure with `match`, then add a win counter.

## Acceptance checks

- Input such as `rust` does not end the program.
- A correct guess prints a win message and exits.
- Comparison logic has unit tests, including equal values.

## Stretch goals

- Let the player choose the upper limit.
- Add a `q` command to quit.
- Offer a second game without restarting the executable.

## Memory check

At the end of each loop turn, which value frees its heap allocation: the parsed `u32` or the input `String`? Why?
