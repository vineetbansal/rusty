[![CI](https://github.com/vineetbansal/rusty/actions/workflows/ci.yml/badge.svg)](https://github.com/vineetbansal/rusty/actions/workflows/ci.yml)

Rusty
=====

A hybrid Rust/Python repository.

### Core Rust Development

```shell
cd rust/
cargo test
```

### Python Wrapper Development

```shell
cd python/
pip install -e .[dev]
pytest
```

### Hybrid Development

If you know what you're doing.
```shell
cd python/
pip install maturin
maturin develop
pytest
```
