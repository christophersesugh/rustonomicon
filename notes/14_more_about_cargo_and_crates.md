# 14. More About Cargo and Crates.io

Cargo is your project manager, build tool, test runner, documentation tool, and package publisher. Start with its everyday commands; the publishing pieces matter only when you are ready to share a library.

## Read `Cargo.toml`

```toml
[package]
name = "task_tracker"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1"
```

This file is a **manifest**. It describes the package, not your program logic. The dependency line means “use a compatible released version of `serde` 1.x.” Cargo records the exact resolved versions in `Cargo.lock` so repeated builds use the same dependency graph.

![The basic Cargo build flow](./diagrams/cargo_project_map.svg)

For an application, commit `Cargo.lock`. For a reusable library, it is conventional to let downstream applications resolve their own lock file; still test your library with a lock file locally.

## Dependencies are compiled code, not magic imports

```bash
cargo add serde --features derive
cargo tree
cargo update
```

- `cargo add` edits the manifest safely.
- Features opt into optional parts of a dependency.
- `cargo tree` shows direct and transitive dependencies.
- `cargo update` refreshes versions within the ranges allowed by `Cargo.toml`.

Cargo downloads crates, checks checksums, resolves compatible versions, compiles dependencies, and caches build outputs in `target/`. Rust code still goes through the same compiler checks whether it came from your folder or a crate.

## Release profiles

```bash
cargo build          # fast-to-build debug profile
cargo build --release # optimized release profile
```

Debug builds favor fast feedback and include extra checks. Release builds spend more compile time optimizing machine code. Always measure before assuming `--release` is necessary, and run tests in the profile that matters for performance-sensitive code.

## Documentation lives with code

```rust
/// Returns the unfinished tasks in display order.
pub fn unfinished(tasks: &[Task]) -> Vec<&Task> {
    tasks.iter().filter(|task| !task.done).collect()
}
```

Triple-slash comments document the public API. Cargo can render them locally:

```bash
cargo doc --open
```

Good library documentation answers: what this item does, what its inputs mean, what it returns, and which failures or invariants matter.

## Workspaces: several related packages together

At a repository root, create a virtual manifest:

```toml
[workspace]
members = ["apps/cli", "crates/task_core"]
resolver = "3"
```

The workspace shares one lock file and one `target/` directory. It is a folder-level convenience; each member remains a separate crate with its own API boundaries.

Useful commands:

```bash
cargo test --workspace
cargo check --workspace
cargo run -p cli
```

## Publishing is a public promise

Before publishing a library to crates.io, add a clear description, license, repository URL, documentation, examples, and tests. Test the exact package contents first:

```bash
cargo package
cargo publish --dry-run
```

Publishing makes a version available to other projects. You cannot replace an already-published version, so treat it as an API promise. For personal experiments, a Git repository or a local path dependency is usually simpler.

## `cargo install` is for command-line programs

```bash
cargo install ripgrep
```

This compiles and installs a binary for you to run from your shell. It is different from adding a dependency to your project; `cargo install` does not change your current `Cargo.toml`.

## Try it now

Run `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo doc --no-deps` in a project you built. These four commands are a solid pre-share checklist.
