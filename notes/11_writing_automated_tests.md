# Writing automated tests

## How to write tests

## Controlling how tests are run
`cargo test` behaves like `cargo run` but run tests in parallel and capture output without displaying outputs, if any. outputs are only displayed for a particular test if it fails.

Command line arguments can be specified to change tests behavior. There are arguments for `cargo test` and for the resultant binary.

### Commands

- `cargo test -- --test-threads=1`: this makes sure tests are run consecutively with a single thread rather than the default behavior of running tests with multi threads.

- `cargo test -- --show-output`: by default, `println!` outputs are not shown in tests, using this commands overrides this behavior to show outputs in passing tests.

- `cargo test <test_name>` or `cargo test <test_module>::<test_name>`: a single test can be ran by specifying the test name or test name from the test module

- `cargo test <sub_test_name>`: this runs multiple tests starting with the same name.

- `cargo test -- --ignored`: this ignores expensive tests containing the `#[ignore]` metadata

- `cargo test -- --include-ignored`: this inculde ignored tests

## Test organization
