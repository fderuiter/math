# Generative Turbulence (Ghost Module)

![Status](https://img.shields.io/badge/status-ghost-lightgrey)
![Domain](https://img.shields.io/badge/domain-applied-blue)

> **"What is hidden is not forgotten, merely waiting for the right environment."**

This module implements deep learning-based turbulence generation using `tch-rs` (LibTorch bindings).

## Install

**Why is this hidden?**
`tch-rs` requires a local installation of LibTorch (the C++ PyTorch backend) and can significantly increase build times and binary sizes. Including it by default would break the "30-Second Rule" for users who just want to run pure math simulations without setting up a deep learning environment.

If you want to experiment with generative turbulence models:

1.  **Install LibTorch:**
    Follow the [tch-rs installation guide](https://github.com/LaurentMazare/tch-rs) to install LibTorch and set the `LIBTORCH` environment variable.

2.  **Add Dependency:**
    Add `tch` to your `Cargo.toml`:
    ```toml
    [dependencies]
    tch = "0.15.0" # Check for latest version
    ```

## Usage

*This module is excluded from compilation by default. You must enable it in the configuration step before using it.*

## Config

**Enable the Module:**
In `math_explorer/src/applied/mod.rs`, uncomment the line:
```rust
// pub mod generative_turbulence;
```

**Contents:**
*   **`models/`**: Variational Autoencoders (VAEs) and GANs for flow field generation.
*   **`networks/`**: Convolutional neural network backbones.
*   **`training.rs`**: Training loops and optimization logic.
*   **`analysis.rs`**: Statistical analysis of generated flow fields (energy spectra, correlation functions).

## Contributing
See the [Project Contributing Guide](../../../../CONTRIBUTING.md) for details on adding new architectures or layers.
