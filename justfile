# Justfile for simcore project

default:
    @just --list

# Lint the entire project
lint:
    cd simcore && cargo clippy --all-features -- -D warnings
    cd simcore_py && cargo clippy --all-features -- -D warnings

# Test the entire project
test:
    cd simcore && cargo test --all-features
    cd simcore_py && cargo test --all-features

# Build Python wheel
build-wheel:
    cd simcore_py && maturin build --release

# Develop mode - installs the wheel in your current environment
develop:
    cd simcore_py && maturin develop --release

# Run the hello_world example
run-hello:
    cd examples/hello_world && cargo run --release

# Create a virtual environment and install the wheel
venv:
    virtualenv .venv
    . .venv/bin/activate && cd simcore_py && maturin develop --release

# Clean build artifacts
clean:
    cd simcore && cargo clean
    cd simcore_py && cargo clean
    cd examples/hello_world && cargo clean 