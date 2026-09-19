# Project 4: Word Frequency Report

**Time:** 1–2 hours
**Practice:** `&str`, `String`, `HashMap`, iterators, and tests

Build a program that receives text and prints each normalized word with its count, ordered from most frequent to least frequent.

Start with pure functions. Add command-line or file input only after those functions work.

## Required behavior

For this text:

```text
Rust is fast. Rust is safe!
```

The report should include `rust: 2`, `is: 2`, `fast: 1`, and `safe: 1`. Treat uppercase and lowercase as the same word; punctuation should not be part of a word.

## Suggested functions

```rust
fn normalize_word(word: &str) -> String
fn count_words(text: &str) -> std::collections::HashMap<String, usize>
fn sorted_counts(counts: &std::collections::HashMap<String, usize>) -> Vec<(&str, usize)>
```

The last function may borrow the map's keys. It must not return those borrowed slices after the map has been dropped.

## Milestones

1. Write tests for `normalize_word` using mixed case and punctuation.
2. Use `split_whitespace` and a `HashMap` to count simple words.
3. Add normalization.
4. Turn the map into a vector and sort by descending count.
5. Print a neat report.

## Acceptance checks

- The input text is not modified.
- Mixed-case copies of the same word count together.
- Tests cover empty input and repeated punctuation.
- Sorting ties have a predictable rule (for example, alphabetical order).

## Stretch goals

- Read text from a path passed on the command line.
- Support Unicode-aware word segmentation after researching the tradeoff; do not pretend `char::is_alphabetic` solves every language perfectly.
- Print the top five words only.

## Memory check

Which data does the `HashMap` own? Which data may `sorted_counts` borrow? Draw the map and result vector before coding it.
