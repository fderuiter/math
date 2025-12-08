# Math Explorer

**Math Explorer** is a comprehensive Rust library and collection of mathematical explorations, ranging from rigorous implementations of physical and mathematical theories to satirical modeling of social dynamics. This repository serves as a playground for exploring algorithms in AI, applied mathematics, climate science, physics, and pure mathematics.

## 📚 Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [AI & Transformers](#ai--transformers)
  - [Applied Mathematics (Favoritism)](#applied-mathematics-favoritism)
  - [Physics (Quantum Coupling)](#physics-quantum-coupling)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [License](#license)

## 🚀 Features

The library is organized into several high-level modules:

### 🤖 AI (`math_explorer::ai`)
- **Transformers**: A full implementation of the Transformer architecture, including:
  - Multi-Head Attention
  - Position-wise Feed-Forward Networks
  - Positional Encoding
  - Encoder/Decoder stacks

### 🛠️ Applied Mathematics (`math_explorer::applied`)
- **Favoritism**: A satirical yet mathematically rigorous model for calculating parental favoritism based on gifts, attention, and other factors.
- **Cannibalism**: Population dynamics models (McKendrick-von Foerster, death rates).
- **Climate (CERA)**: Implementation of the CERA (Climate-invariant Encoding through Representation Alignment) framework using autoencoders.
- **GRPO**: Group Relative Policy Optimization formulas.
- **Win Ratio**: Statistical methods for win ratio analysis in clinical trials.
- **Pharmacokinetics**: Models for drug concentration over time (e.g., Adderall).
- **LoraHub**: Core mathematical operations for combining LoRA (Low-Rank Adaptation) modules.

### 🌌 Physics (`math_explorer::physics`)
- **Quantum Mechanics**: Calculation of Clebsch-Gordan coefficients for angular momentum coupling.
- **Astrophysics**: Empirical formulas for estimating properties of irregular dwarf galaxies.

### 📐 Pure Mathematics (`math_explorer::pure_math`)
- **Number Theory**: Prime generation, primality testing, and partition functions.
- **Graph Theory**: Graph parameters like degeneracy and approximate vertex cover.
- **Elliptic Curves**: Divisibility of coefficients of modular polynomials.
- **Algorithmic Information**: Kolmogorov complexity approximations and combinatorial lemmas.

## 📋 Prerequisites

To work with this repository, you need to have **Rust** and **Cargo** installed.

- **Rust**: The programming language used.
- **Cargo**: The package manager and build tool for Rust.

You can install Rust and Cargo via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🛠️ Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/math-explorer.git
    cd math-explorer
    ```

2.  **Build the project:**

    Navigate to the library directory and build:

    ```bash
    cd math_explorer
    cargo build --release
    ```

## 💻 Usage

Here are a few examples of how to use the library in your Rust code.

### AI & Transformers

```rust
use math_explorer::ai::transformer::Transformer;
use nalgebra::DMatrix;

// Initialize a small Transformer model
// (num_layers, d_model, heads, d_feed_forward)
let transformer = Transformer::new(2, 512, 8, 2048);

// Create dummy input (sequence_length=10, d_model=512)
let input = DMatrix::zeros(10, 512);

// Forward pass through the encoder
let encoded = transformer.encoder.forward(input, None);
```

### Applied Mathematics (Favoritism)

Calculate who the favorite child is based on a complex set of inputs!

```rust
use math_explorer::applied::favoritism::{FavoritismInputs, calculate_favoritism_score};

let mut inputs = FavoritismInputs::default();
inputs.wealth = 10.0;             // High wealth
inputs.helped_during_crisis = true; // Very helpful

let score = calculate_favoritism_score(&inputs);
println!("Favoritism Score: {}", score);
```

### Physics (Quantum Coupling)

Calculate Clebsch-Gordan coefficients: $\langle j_1 m_1; j_2 m_2 | J M \rangle$.

```rust
use math_explorer::physics::quantum::clebsch_gordan;

let coeff = clebsch_gordan(1.5, -0.5, 1.0, 1.0, 2.5, 0.5);
println!("Clebsch-Gordan Coefficient: {}", coeff);
```

## 🧪 Testing

The project includes a comprehensive test suite. To run the tests:

```bash
cd math_explorer
cargo test
```

This will compile the library and run all unit tests defined in the modules, ensuring that algorithms (from quantum coupling to battery degradation) are working correctly.

## 📂 Project Structure

```
math_explorer/
├── src/
│   ├── ai/              # Transformer and Attention mechanisms
│   ├── applied/         # Applied math (Climate, Favoritism, etc.)
│   ├── climate/         # CERA framework specific implementation
│   ├── physics/         # Quantum and Astrophysics
│   ├── pure_math/       # Number Theory, Graph Theory, etc.
│   └── lib.rs           # Library entry point
├── tests/               # Integration tests
├── Cargo.toml           # Dependencies and package info
└── README.md            # Library documentation
```

## 📄 License

This project is open-source. Please refer to the repository for license details.
