# 7. Packages, Crates, and Modules

As a program grows, names need an address. Rust uses packages, crates, modules, and paths to say exactly where code lives and who may use it.

## Four words, one picture

```text
package (one Cargo.toml)
└── crate (one thing Cargo compiles)
    └── module (a named code area)
        └── item (a function, struct, enum, trait, ...)
```

- A **package** is a folder managed by Cargo. It has one `Cargo.toml`.
- A **crate** is the compilation unit. `src/main.rs` is a binary crate; `src/lib.rs` is a library crate.
- A **module** groups names inside a crate.
- A **path** names an item, such as `crate::garden::vegetables::Asparagus`.

A package may contain one library crate and one or more binary crates. This lets a command-line program keep its reusable logic in `src/lib.rs` and only argument parsing in `src/main.rs`.

## Start private, make public deliberately

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

Everything here is private to its parent module by default. This is intentional: changing private code cannot break another crate. Use `pub` to expose the smallest useful part:

```rust
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

`pub` lets code travel *through* a module boundary. A public child inside a private parent is still unreachable from outside the private parent.

## Import a convenient local name with `use`

```rust
use std::collections::HashMap;
use std::fmt::{self, Display};

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ada", 10);
}
```

`use` does not copy code or run anything. It gives an existing item a shorter name in this scope. `use std::fmt::{self, Display};` imports both the `fmt` module and its `Display` trait.

### Paths can be relative

- `crate::...` starts at this crate's root.
- `self::...` starts at the current module.
- `super::...` starts one module up.

Think of `crate` as the project-wide import root, similar to an explicit alias in a TypeScript project. Rust avoids a global namespace: every item is resolved in a known module tree.

## Split a module into files

For a library with `src/lib.rs`:

```text
src/
├── lib.rs
└── garden/
    ├── mod.rs
    └── vegetables.rs
```

`src/lib.rs`:

```rust
pub mod garden;
```

`src/garden/mod.rs`:

```rust
pub mod vegetables;
```

`src/garden/vegetables.rs`:

```rust
pub struct Asparagus;
```

The modern alternative to `garden/mod.rs` is `garden.rs` next to a `garden/` folder. Pick one convention and stay consistent; the module declaration, not the folder alone, makes a module exist.

## `pub use`: expose a simpler public API

```rust
mod internal {
    pub mod parser {
        pub struct Config;
    }
}

pub use internal::parser::Config;
```

Users of this library can now write `my_crate::Config`. Re-exporting lets you reorganize internal folders without forcing users to change every import.

## What happens at runtime?

Modules are mostly an organizing and privacy tool for the compiler. They do not create JavaScript-like module objects at runtime. The compiler resolves names while compiling, checks visibility, and produces one executable or library artifact.

## Try it now

Take a one-file program and move its data type plus helper functions into `src/lib.rs`. Keep `main.rs` as a short caller. That is the shape used by the [minigrep project](./12_io_project.md).
