# Math Explorer Crate

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Crate](https://img.shields.io/crates/v/math_explorer.svg)

The core library for the **Math Explorer** project. This crate provides a collection of mathematical algorithms and models, implemented from first principles in Rust.

> **Note:** This is the inner library crate. For the full repository context, see the [Project Root README](../README.md).

## Install

Add this crate to your dependencies:

```toml
[dependencies]
math_explorer = { path = "path/to/math_explorer" }
```

## Usage

The library is organized into high-level domains:

- **`ai`**: Artificial Intelligence primitives, including Transformer components (Attention, Encoder/Decoder), NeRF-Diffusion, and Self-Calibration loops.
- **`applied`**: Mathematical models applied to specific domains like **Clinical Trials** (Win Ratio), **Game Theory**, **Favoritism** (Satire), and **Isosurface Extraction**.
- **`biology`**: Biological modeling, including **Neuroscience** (Hodgkin-Huxley) and **Morphogenesis**.
- **`climate`**: Climate modeling tools, featuring the **CERA** autoencoder framework.
- **`epidemiology`**: Disease modeling, from standard **SIR/SEIR** compartmental models to stochastic network dynamics.
- **`physics`**: Simulations of physical systems, including **Quantum Mechanics**, **Fluid Dynamics**, **Chaos Theory** (Lorenz System), and **MRI Physics**.
- **`pure_math`**: Foundational mathematics, covering **Algebra**, **Number Theory** (Partitions, Q-Series), **Graph Theory**, and **Differential Geometry**.

### Example: Computational Biology (Neuroscience)

Simulate a neuron's membrane potential using the Hodgkin-Huxley model:

```bash
# Run the simulation
cargo run --example hodgkin_huxley_demo
```

*(See `examples/hodgkin_huxley_demo.rs` for implementation details)*

### Example: Chaos Theory

Calculate the Lyapunov exponent for a Lorenz System:

```bash
# Run the simulation
cargo run --example lorenz_chaos
```

*(See `examples/lorenz_chaos.rs` for implementation details)*

### Example: AI Transformer

Initialize a Transformer Encoder stack:

```bash
# Run the simulation
cargo run --example transformer_demo
```

*(See `examples/transformer_demo.rs` for implementation details)*

## Config

Some modules are intentionally excluded from the default compilation to reduce build times or avoid heavy external dependencies (like LibTorch). These are called "Ghost Modules."

### Generative Turbulence
A deep learning-based module for generating turbulent flow fields using `tch-rs`. It is excluded by default because it requires a local LibTorch installation.

To learn how to enable it and explore the code, see the [Generative Turbulence Documentation](src/applied/generative_turbulence/README.md).

## Contributing

Please refer to [CONTRIBUTING.md](../CONTRIBUTING.md) in the project root. To run the comprehensive test suite for all modules:

```bash
cargo test
```
