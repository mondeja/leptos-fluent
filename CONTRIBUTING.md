# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

### Unit tests

Unit tests are located at _tests/src/_ and on each crate.

```sh
cargo test -p leptos-fluent -p leptos-fluent-macros -p leptos-fluent-tests
```

### End to end tests

End to end tests are defined in different folders because they need to be run in
different environments.

#### CSR

End to end tests that only need a CSR environment are located at _tests/end2end/csr/_.

##### Run all

To install the dependencies, execute:

```sh
cargo install wasm-pack wasm-pack-test-all
```

Make sure that you've a browser compatible with [`wasm-pack test`] installed.
Then execute:

```sh
wasm-pack-test-all tests/end2end/csr --{browser} --headless
```

where `{browser}` is one of `firefox`, `chrome`, or `safari`.

##### Run a specific test

There are two type of tests, those which need custom browser setups and those
which don't.

###### Custom browser setups

Those that are defined inside their own crate (for example `set-language-to-cookie`),
are located in their own folder because they need custom browser setup. See
the file _webdriver.json_ inside each folder for more information.

To run one of those tests use [`wasm-pack test`] directly. For example, to run the
`set-language-to-cookie` test suite run:

```sh
wasm-pack test --firefox --headless tests/end2end/csr/set-language-to-cookie
```

###### Simple tests

Those that are defined inside the folder _tests/end2end/csr/tests_ don't need
custom browser setups. To run all those tests at once use:

```sh
wasm-pack-test-all tests/end2end/csr --{browser} --headless
```

To run one of those tests use [`wasm-pack test`] directly and pass it in the
`--test` flag. For example, to run the `csr_complete` test suite run:

```sh
wasm-pack test --firefox --headless tests/end2end/csr --test csr_complete
```

[`wasm-pack test`]: https://rustwasm.github.io/docs/wasm-pack/commands/test.html

#### SSR

> Only available in Unix-like systems.

End to end tests that need SSR environments are located at _tests/end2end/ssr/_.

- Test functions are located at _tests/end2end/ssr/src/lib.rs_.
- Apps used by the tests are located at _tests/end2end/ssr/src/apps/_.

##### Run all

To run all the tests, use:

```sh
cargo test -p leptos-fluent-end2end-ssr-tests
```

##### Run specific tests

To run specific tests, pass their test functions names to the environment variable
`TESTS`. For example:

```sh
TESTS=initial_language_from_accept_language_header_axum cargo test -p leptos-fluent-end2end-ssr-tests
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
