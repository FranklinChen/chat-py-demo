# Demo of CHAT text and AST processing through Python

This is a demo of creating a JSON representation of a CHAT document according
to a JSON Schema, passing it to a Rust library that deserializes the JSON into
a Rust AST, then serializes the AST to CHAT text, which is then returned back to
Python.

The JSON Schema is generated from the annotated Rust AST code by
means of the [`schemars`](https://docs.rs/schemars/latest/schemars/) crate.

Then a Python [`pydantic` v2](https://docs.pydantic.dev/latest/) module is
generated from that JSON Schema by means of [`datamodel-code-generator`](https://koxudaxi.github.io/datamodel-code-generator/).

## Set up Rust

Make sure the [standard toolchain for Rust](https://rustup.rs/) is installed.

Run the Rust tests to verify that the pure-Rust part of the library works:

```
$ cargo test
```

## Set up Python

First, set up a Python virtual environment. I used
Python 3.12, but slightly older versions may also work.
```
$ python3.12 -m venv .venv
$ source .venv/bin/activate
```

Install [`maturin`](https://www.maturin.rs/) and some other development tools:
```
$ pip install maturin
$ pip install datamodel-code-generator
$ pip install pytest
```

Generate the JSON Schema and the Pydantic module before proceeding further:
```
$ make clean all
```

Build the `chat_py_demo` Python module (which also compiles Rust):
```
$ maturin develop
```

Run Python tests to verify that things work:
```
$ pytest python/tests
```
