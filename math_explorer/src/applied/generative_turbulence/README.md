# Generative Turbulence (Ghost Module)

> **Status:** 👻 Excluded from compilation.

## Why is this hidden?

This module implements deep learning-based turbulence generation using `tch-rs` (LibTorch bindings).

**The Problem:** `tch-rs` requires a local installation of LibTorch (the C++ PyTorch backend) and can significantly increase build times and binary sizes. Including it by default would break the "30-Second Rule" for users who just want to run pure math simulations without setting up a deep learning environment.

## How to Enable

If you want to experiment with generative turbulence models:

1.  **Install LibTorch:**
    Follow the [tch-rs installation guide](https://github.com/LaurentMazare/tch-rs) to install LibTorch and set the `LIBTORCH` environment variable.

2.  **Add Dependency:**
    Add `tch` to your `Cargo.toml`:
    ```toml
    [dependencies]
    tch = "0.15.0" # Check for latest version
    ```

3.  **Uncomment the Module:**
    In `math_explorer/src/applied/mod.rs`, uncomment the line:
    ```rust
    // pub mod generative_turbulence;
    ```

## Contents

*   **`models/`**: Variational Autoencoders (VAEs) and GANs for flow field generation.
*   **`networks/`**: Convolutional neural network backbones.
*   **`training.rs`**: Training loops and optimization logic.
*   **`analysis.rs`**: Statistical analysis of generated flow fields (energy spectra, correlation functions).
