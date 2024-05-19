# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

[!NOTE] End to end tests are located at _end2end/_.

### End to end tests

You need to install a browser and run:

```bash
cd tests
wasm-pack test --{browser} --headless
```

Where `{browser}` is one of `firefox`, `chrome`, or `safari`. For example:

```sh
cd end2end
wasm-pack test --firefox --headless
```

If you want to run a test suite:

```sh
cd end2end
wasm-pack test --firefox --headless --test csr_minimal
```

### Unit tests

```sh
cargo test
```

## Documentation

```sh
cargo doc --open --no-deps
```
