# SimCore

A Rust-based mission/campaign simulation model that's interfaced with through a Python SDK.

## Overview

SimCore provides a high-performance simulation engine for military mission and campaign modeling. It combines:

- A Rust kernel for efficient computation
- Python bindings for easy analyst use
- Discrete event simulation for accurate timing
- Entity Component System (ECS) architecture for flexible modeling

## Installation

### Dependencies

- Rust 1.78+ with `rustup target add x86_64-unknown-linux-gnu`
- Python 3.10+
- Maturin ≥ 1.5
- Virtualenv (optional for isolated environments)

### Installation from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/simcore.git
cd simcore

# Install Rust dependencies and build the wheel
just build-wheel

# Install in development mode
just develop
```

### Installation via pip (once published)

```bash
pip install simcore
```

## Usage

### Basic Example

```python
from simcore import Scenario

# Create a simulation with two aircraft
scn = (Scenario(duration=300)
       .add_aircraft("F-16", x=0, y=0, z=8000, hdg=90, ktas=420)
       .add_aircraft("Su-35", x=150000, y=0, z=8500, hdg=270, ktas=430))

# Run the simulation
result = scn.run(trials=1)

# Display positions
print(result.positions)
```

See the `python_examples` directory for more detailed examples, including a Jupyter notebook.

## Project Structure

```
simcore/         # Core simulation engine
simcore_py/      # Python bindings
examples/        # Rust examples
python_examples/ # Python/Jupyter examples
```

## Development

This project uses [just](https://github.com/casey/just) as a command runner:

```bash
# Show available commands
just

# Run tests
just test

# Run linting
just lint

# Build Python wheel
just build-wheel

# Development install
just develop

# Create a virtual environment
just venv
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
