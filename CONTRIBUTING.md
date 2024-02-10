# Contributing

## Setup

```bash
python3 -m pip install pre-commit
pre-commit install
```

## Tests

You need to install a browser and run:

```bash
cd leptos-fluent
wasm-pack test --{browser} --headless --features tests
```

Where `{browser}` is one of `firefox`, `chrome`, or `safari`. See `wasm-pack test --help` for more information.
