# test-toolbox
Utility library of helper macros for working with unit tests.

### Macros

* `actual!` - declare actual variable with differing `debug` and `release` syntax
* `expect!` - declare expected variable with differing `debug` and `release` values
* `capture!` - captures `stdout` and `stderr` for testing output

## Resources
* [Docs](https://docs.rs/test-toolbox/0.5.0/test_toolbox/) for more detailed information

## Usage

Each macro is gated by a feature.

No feature is mutually exclusive and can be combined as needed.

* `actual!` macro
```toml
[dependencies]

test-toolbox = { version = "0.5", features = ["actual"] }
```

* `capture!` macro
```toml
[dependencies]

test-toolbox = { version = "0.5", features = ["capture"] }
```

* `expect!` macro
```toml
[dependencies]

test-toolbox = { version = "0.5", features = ["expect"] }
```

## Implemented
* [x] `actual!` macro
* [x] `expect` macro
* [x] `capture!` macro
