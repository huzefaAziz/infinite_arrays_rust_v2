# InfiniteArrays (Rust)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/huzefaAziz/infinite_arrays_py)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

A Rust library for representing arrays with infinite dimension sizes, designed for numerical computing. Infinite arrays are by necessity lazy, and so this library provides lazy evaluation for efficient computation.

This library is converted from the Python version [infinite_arrays_py](https://github.com/huzefaAziz/infinite_arrays_py), which was inspired by [InfiniteArrays.jl](https://github.com/JuliaArrays/InfiniteArrays.jl).

## Features

- **Infinite Arrays**: Create arrays with infinite dimensions
- **Lazy Evaluation**: Values are computed on-demand for efficiency
- **Broadcasting Support**: Supports element-wise operations and broadcasting
- **Flexible Indexing**: Support for various infinite range types
- **Mutable Caching**: Convert lazy arrays to cached (mutable) versions
- **Infinite-dimensional QR Algorithm**: Compute spectra of infinite operators with convergence control
- **CMake Support**: Build with CMake for integration with C/C++ projects

## Building with CMake

### Prerequisites

- Rust toolchain (1.70 or later)
- CMake (3.15 or later)
- Cargo (comes with Rust)

### Build Instructions

```bash
# Create build directory
mkdir build
cd build

# Configure with CMake
cmake ..

# Build
cmake --build .

# Run examples
./example_basic
./example_iqr
```

### CMake Options

The CMake build system supports:
- Building shared and static libraries
- Running tests: `make cargo_test`
- Generating documentation: `make cargo_doc`
- Cross-platform support (Windows, Linux, macOS)

## Building with Cargo

Alternatively, you can build directly with Cargo:

```bash
# Build library
cargo build --release

# Run examples
cargo run --example basic_usage
cargo run --example iqr_example

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Quick Start

### Basic Usage

```rust
use infinite_arrays::*;
use infinite_arrays::arrays::Shape;
use std::sync::Arc;

// Create an infinite vector of ones
let x = Arc::new(Ones::new(None));
println!("x[0] = {}", x.get(0));  // 1.0
println!("x[5] = {}", x.get(5));  // 1.0

// Create infinite diagonal matrix
let d = InfiniteDiagonal::new(|i| (i + 1) as f64);
println!("D[0, 0] = {}", d.get(0, 0));  // 1.0
println!("D[0, 1] = {}", d.get(0, 1));  // 0.0

// Broadcasting operations
let result = BroadcastArray::new(
    |i| (-(i + 1) as f64).exp() + 2.0,
    Shape::OneD(Some(INFINITY)),
);
println!("result[0] = {}", result.get(0));

// Create cached (mutable) array
let c = cache(x.clone());
c.set(0, 3.0);
println!("C[0] = {}", c.get(0));  // 3.0
```

### Infinite-dimensional QR Algorithm

```rust
use infinite_arrays::iqr::*;
use num_complex::Complex64;

// Create a diagonal operator
let diag_op = create_diagonal_operator(|i| Complex64::new((i + 1) as f64, 0.0));

// Compute spectrum
let result = iqr_algorithm(&diag_op, 50, 1000, 1e-10, None, false);
println!("Eigenvalues: {:?}", result.eigenvalues);
println!("Converged: {}", result.converged);
```

## API Reference

### Core Types

#### `Ones`, `Zeros`, `Fill`

Create infinite arrays filled with constant values.

#### `InfiniteDiagonal`

Create an infinite diagonal matrix with values from a function.

#### `BroadcastArray`

Lazy broadcasted array that computes values on-demand using a function.

#### `CachedArray`

Cached (mutable) version of an infinite array.

### Range Types

#### `OneToInf`

Infinite range starting from 1: 1, 2, 3, ...

#### `InfUnitRange`

Infinite unit range starting from a given value.

#### `InfStepRange`

Infinite step range with specified start and step.

### Infinite-dimensional QR Algorithm

The library includes an implementation of the infinite-dimensional QR (IQR) algorithm for computing spectra of infinite-dimensional operators, as described in:

> Colbrook, M.J. & Hansen, A.C. "On the infinite-dimensional QR algorithm"  
> Numer. Math. 143, 17-83 (2019). <https://doi.org/10.1007/s00211-019-01047-5>

#### Functions

- `iqr_algorithm`: Compute eigenvalues and optionally eigenvectors
- `iqr_spectrum`: Compute spectrum with adaptive truncation
- `create_diagonal_operator`: Create a diagonal operator
- `create_tridiagonal_operator`: Create a tridiagonal operator

## Examples

See the `examples/` directory for complete examples:
- `basic_usage.rs`: Basic array operations
- `iqr_example.rs`: IQR algorithm usage

## Dependencies

- `ndarray`: N-dimensional arrays
- `nalgebra`: Linear algebra
- `num-complex`: Complex number support
- `num-traits`: Numeric traits

## Limitations

- Infinite arrays cannot be converted to finite arrays directly
- Length operations are not supported for infinite arrays
- Some operations may raise errors when attempting to materialize infinite arrays

## Development

To contribute to this project:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This library is converted from the Python version [infinite_arrays_py](https://github.com/huzefaAziz/infinite_arrays_py), which was inspired by [InfiniteArrays.jl](https://github.com/JuliaArrays/InfiniteArrays.jl).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

