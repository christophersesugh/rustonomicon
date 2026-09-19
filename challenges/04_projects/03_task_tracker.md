# Project 3: In-Memory Task Tracker

**Time:** 2–4 hours
**Practice:** structs, enums, methods, `Vec`, borrowing, and modules

Build a command-line task tracker. It starts empty each run; persistence is a stretch goal. This keeps the project focused on modeling data well before adding file I/O.

## Data model

Start with something close to this, then make names your own:

```rust
struct Task {
    id: u32,
    title: String,
    status: Status,
}

enum Status {
    Todo,
    Done,
}
```

Create a `TaskList` that owns `Vec<Task>`. Give it methods to add, list, complete, and remove tasks.

## Required behavior

```text
add Buy milk
list
done 1
remove 1
quit
```

1. Assign each new task a unique id.
2. Store an owned `String` title so user input remains valid after the command is processed.
3. Listing must not consume the task list.
4. Marking done must change exactly one matching task or return a useful error.
5. Split parsing and task-list logic into separate modules or files.

## Milestones

1. Create `Task`, `Status`, and a `TaskList::add` method with unit tests.
2. Add `list` and display status with `match`.
3. Add `complete(id)` returning `Result<(), String>`.
4. Add a loop that parses terminal commands.
5. Move model code into `src/tasks.rs` and expose only the types `main` needs.

## Acceptance checks

- `add` followed by `list` shows the task.
- Completing an unknown id returns an error, not a panic.
- Unit tests cover add, complete, and remove.
- `TaskList` owns its tasks; it does not store references to temporary command input.

## Stretch goals

- Add priority with an enum.
- Filter `list` by todo or done.
- Save to a text file after Chapter 12.

## Memory check

Why should `Task.title` be `String` rather than `&str` when it comes from a `String` used to read one command?
