# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

### End to end tests

- End to end tests are located at _tests/end2end/_

```sh
cargo install wasm-pack
```

You need a browser installed. Run:

```sh
cd tests/end2end
wasm-pack test --{browser} --headless
```

where `{browser}` is one of `firefox`, `chrome`, or `safari`. For example:

```sh
cd tests/end2end
wasm-pack test --firefox --headless
```

To run a test suite pass it in the `--test` flag.
For example, to run the `csr_complete` test suite run:

```sh
cd tests/end2end
wasm-pack test --firefox --headless --test csr_complete
```

To run all tests with custom _webdriver.json_ files execute
_tests/end2end/run.sh_.

### Unit tests

Unit tests are located at _tests/src/_ and on each crate.

```sh
cargo test -p leptos-fluent -p leptos-fluent-macros -p leptos-fluent-tests
```

## Documentation

### Reference

```sh
cargo doc --open --no-deps
```

### Book

```sh
cargo install mdbook
```

```sh
cd book
mdbook serve --open
```
