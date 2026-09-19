# Project 6: Parallel Log Analyzer

**Time:** 2–4 hours
**Practice:** closures, channels, threads, `Arc`, `Mutex`, and ownership across threads

Build a program that counts log levels across several in-memory log chunks in parallel. Keep the input in memory at first; opening many real files is a separate I/O problem.

Example input chunks:

```text
INFO started
WARN cache missed
ERROR database unavailable
```

The final report should show how many `INFO`, `WARN`, and `ERROR` lines occurred.

## Required behavior

1. Write `count_levels(text: &str) -> Counts` as a single-threaded, well-tested function.
2. Split a `Vec<String>` of chunks among worker threads.
3. Each worker sends its local `Counts` through an `mpsc` channel.
4. The main thread combines received counts and prints the result.
5. Do not share a mutable counter until the channel solution works.

## Milestones

1. Model `Counts` as a struct with an `add` method.
2. Make the pure counting tests pass.
3. Spawn one thread that owns one `String` chunk and sends its result.
4. Spawn one worker per chunk and receive every result.
5. Replace unbounded thread creation with a fixed-size worker pool only as a stretch goal.

## Acceptance checks

- Empty chunks work.
- The main thread joins every worker.
- No worker borrows a local variable that can disappear before the thread runs.
- Tests cover single-threaded counting; the threaded version produces the same final counts.

## Stretch goals

- Use `Arc<Mutex<Counts>>` instead of a result channel and explain why both solutions are safe.
- Add a `--workers 4` flag and build a real bounded worker pool.
- Record unknown log levels.

## Memory check

When you add `move` to a thread closure, which values are transferred into the closure? Which values must use `Arc` because more than one thread needs ownership?
