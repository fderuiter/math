# A Collection of Mathematical Explorations

Welcome to this repository, a personal collection of mathematical explorations that range from the serious to the satirical. This project houses a variety of content, including a Rust library for mathematical concepts, academic papers, and humorous essays. The goal is to explore different facets of mathematics in a creative and programmatic way.

## Components

This repository is composed of several key parts:

- **`math_explorer`**: A Rust library containing various mathematical algorithms and concepts.
- **`essay.tex`**: A satirical paper that presents a mathematical model for calculating maternal favoritism.
- **`tensors.tex`**: A serious mathematical proof on the coupling of irreducible spherical tensors.
- **`favorite_child.md`**: An intentionally empty file, adding to the satire of the favoritism essay.

### The `math_explorer` Library

The `math_explorer` crate is a Rust library with the following modules:

- **`algebra`**: Contains concepts related to algebra.
- **`number_theory`**: Contains concepts related to number theory.
- **`quantum`**: Implements quantum mechanical calculations, such as Clebsch-Gordan coefficients, which are discussed in `tensors.tex`.
- **`favoritism`**: A Rust implementation of the satirical favoritism formula described in `essay.tex`.

## Building and Testing

To build and test the `math_explorer` library, navigate to the `math_explorer` directory and use the following Cargo commands:

```bash
cd math_explorer
cargo build
cargo test
```
