# Rustonomicon Challenges

These challenges are practice, not a second textbook. First read the matching note, then write code from memory. Getting stuck is part of learning: reduce the problem, read the compiler message, and try again before checking a hint.

## Two complementary tracks

| Track | Purpose | How to use it |
| --- | --- | --- |
| Topic drills | Learn one Rust idea in a small, testable function. | Replace each `todo!()` and run the file's tests. |
| Build projects | Combine several ideas into a program someone could actually use. | Build the project in its own Cargo folder and meet its acceptance checks. |

The topic drills are arranged in this order:

- **[01_beginner](./01_beginner/):** variables, types, functions, ownership, borrowing, and slices.
- **[02_intermediate](./02_intermediate/):** structs, enums, collections, errors, traits, generics, and tests.
- **[03_advanced](./03_advanced/):** lifetimes, closures, iterators, smart pointers, concurrency, and patterns.
- **[04_projects](./04_projects/):** seven small applications that build recall and design skill.

## A practice rhythm that builds memory

1. Read the matching note and type one example yourself.
2. Solve a drill without copying the example.
3. Run the test immediately.
4. Explain the error or fix in your own words.
5. Build the related mini-project after two or three topics.
6. Re-do one old drill a few days later without looking at your previous solution.

Tests show whether a result is correct; they do not prove you understand it. After a passing test, ask: “who owns this value, and when is it dropped?”

## Running a topic drill

From the challenge's folder, compile and run its tests:

```bash
rustc --test 01_variables.rs -o variables_tests
./variables_tests
```

The `-o variables_tests` name avoids leaving an executable that clashes with the source filename. Remove only the generated test binary when you are done; never edit the test assertions just to make a failing solution pass.

## Starting a build project

Each project brief states the problem, skills, milestones, acceptance checks, and stretch goals. Create a separate project for it so you practise the real Cargo loop:

```bash
cargo new my_project_name
cd my_project_name
cargo run
```

Keep your solution code separate from these briefs. That makes it easy to revisit a project later and build it again from scratch.
