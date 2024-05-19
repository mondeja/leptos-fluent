# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

[!NOTE] Tests are located at _tests/src/web.rs_.

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

## Documentation

```sh
cargo doc --open --no-deps
```
