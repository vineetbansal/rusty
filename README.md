[![CI](https://github.com/vineetbansal/rusty/actions/workflows/ci.yml/badge.svg)](https://github.com/vineetbansal/rusty/actions/workflows/ci.yml)

Rusty
=====

A hybrid Rust/Python repository.

### Core Rust Development

```shell
cargo test
```

### Python Wrapper Development

```shell
pip install -e .[dev]
pytest
```

### Hybrid Development

If you know what you're doing.
```shell
pip install maturin
maturin develop
pytest
```
