# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

### End to end tests

End to end tests are located at _tests/end2end/_. To install the dependencies, run:

```sh
cargo install wasm-pack wasm-pack-test-all
```

Make sure that you've a browser compatible with [`wasm-pack test`] installed.
Then run:

```sh
wasm-pack-test-all tests/end2end --{browser} --headless
```

where `{browser}` is one of `firefox`, `chrome`, or `safari`. For example:

```sh
wasm-pack-test-all tests/end2end --firefox --headless
```

To run a test use [`wasm-pack test`] directly and pass it in the `--test` flag.
For example, to run the `csr_complete` test suite run:

```sh
wasm-pack test --firefox --headless tests/end2end --test csr_complete
```

[`wasm-pack test`]: https://rustwasm.github.io/wasm-pack/book/commands/test.html

### Unit tests

Unit tests are located at _tests/src/_ and on each crate.

```sh
cargo test -p leptos-fluent -p leptos-fluent-macros -p leptos-fluent-tests
```

## Documentation

### Reference

```sh
cargo doc --open --no-deps -p leptos-fluent -p leptos-fluent-macros
```

### Book

```sh
cargo install mdbook
```

```sh
mdbook serve book --open
```
