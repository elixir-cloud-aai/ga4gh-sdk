## GA4GH-SDK bindings to python

https://docs.rs/pyo3/latest/pyo3

Install maturin:

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin
pip install pytest
```

## Develop

Start in dev:

```bash
maturin develop
```

Test the python lib:

```bash
pytest
```

## Build

Build the wheel:

```bash
maturin build
```