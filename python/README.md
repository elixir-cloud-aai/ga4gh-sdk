## GA4GH-SDK Python Bindings

This repository provides Python bindings for the GA4GH-SDK using [PyO3](https://docs.rs/pyo3/latest/pyo3).

## Prerequisites

- Python 3.12
- [Maturin](https://github.com/PyO3/maturin)
- [PyTest](https://docs.pytest.org/en/stable/)

## Installation

1. **Create a virtual environment:**

    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    ```

2. **Install dependencies:**

    ```bash
    pip install maturin pytest
    ```

## Development

To start development, use the following command to build and install the package in development mode:

```bash
maturin develop
```

## Build

Build the Python library wheel:

```bash
maturin build
```

## Running Tests

To run the lib tests, use:

```bash
pytest
```

## Documentation

For more information on PyO3, visit the [PyO3 documentation](https://docs.rs/pyo3/latest/pyo3).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
