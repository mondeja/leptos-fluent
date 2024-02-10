# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

You need to install a browser and run:

```bash
cd tests
wasm-pack test --{browser} --headless
```

Where `{browser}` is one of `firefox`, `chrome`, or `safari`. See `wasm-pack test --help` for more information.

[!NOTE] Tests are located at _tests/src/web.rs_.
