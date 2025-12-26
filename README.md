# Math Explorer

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Maintenance](https://img.shields.io/badge/maintained%3F-yes-brightgreen.svg)

**Math Explorer** is a comprehensive Rust library that bridges the gap between rigorous academic theory and executable code. From simulating **Quantum Mechanics** to modeling **Social Favoritism**, this repository serves as a verifiable playground for complex algorithms.

> **"Code explains HOW; Docs explain WHY."**

---

## ⚡ Quickstart (30 Seconds)

Get up and running immediately.

### 1. Install
Add `math_explorer` to your project (or clone the repo):

```bash
git clone https://github.com/fderuiter/math-explorer.git
cd math-explorer/math_explorer
cargo build --release
```

### 2. Run "Hello World" (Quantum Physics)
Calculate Clebsch-Gordan coefficients for angular momentum coupling:

```rust
use math_explorer::physics::quantum::clebsch_gordan;

fn main() {
    // Coupling j1=1.5, m1=-0.5 with j2=1.0, m2=1.0 to J=2.5, M=0.5
    let coeff = clebsch_gordan(1.5, -0.5, 1.0, 1.0, 2.5, 0.5);
    println!("Clebsch-Gordan Coefficient: {:.4}", coeff);
}
```

---

## 📚 Table of Contents

- [Features](#-features)
- [Deep Dive: Modules](#-deep-dive-modules)
- [Testing](#-testing)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Features

Math Explorer is organized into high-level domains, each solving specific problems:

| Domain | Module | Description |
| :--- | :--- | :--- |
| **🤖 AI** | `math_explorer::ai` | Transformers (Attention, Encoders), NeRF-Diffusion (SDS), and Self-Calibration loops. |
| **🛠️ Applied** | `math_explorer::applied` | **Favoritism** (Satirical modeling), **Clinical Trials** (Win Ratio), and **LoraHub**. |
| **🧬 Biology** | `math_explorer::biology` | **Neuroscience** (Hodgkin-Huxley), **Kinetics** (Michaelis-Menten), and **Morphogenesis** (Turing Patterns). |
| **🌍 Climate** | `math_explorer::climate` | **CERA Framework** for climate-invariant machine learning and auto-encoding. |
| **🦠 Epidemiology** | `math_explorer::epidemiology` | **SIR/SEIR Models**, Network propagation, and Stochastic dynamics. |
| **🌌 Physics** | `math_explorer::physics` | Quantum Mechanics (Clebsch-Gordan), Astrophysics, Chaos Theory (Lorenz System), and Fluid Dynamics. |
| **📐 Pure Math** | `math_explorer::pure_math` | Number Theory (Partitions), Graph Theory, and Differential Geometry. |

---

## 🔍 Deep Dive: Modules

### 🤖 Artificial Intelligence
Implement state-of-the-art architectures from scratch.

**Example: Transformer Encoder**
```rust
use math_explorer::ai::transformer::Encoder;
use nalgebra::DMatrix;

// Initialize an Encoder stack: 2 layers, 512 embedding dim, 8 heads, 2048 FF dim
let encoder = Encoder::new(2, 512, 8, 2048);

// Dummy input: Sequence length 10
let input = DMatrix::zeros(10, 512);
let encoded = encoder.forward(input, None);
```

### 🛠️ Applied Mathematics: Favoritism
A "rigorous" mathematical model to determine who the favorite child is.

```rust
use math_explorer::applied::favoritism::{FavoritismInputs, calculate_favoritism_score};

let mut inputs = FavoritismInputs::default();
inputs.personality.wealth = 10.0;          // High wealth factor
inputs.social.helped_during_crisis = true; // High social utility

let score = calculate_favoritism_score(&inputs);
println!("Favoritism Score: {}", score); // Higher is better
```

### 🌌 Physics: Chaos Theory
Explore the Lorenz System and Lyapunov exponents.

*(See `math_explorer/src/physics/chaos/mod.rs` for implementation details)*

---

## 🧪 Testing

We rely on standard Rust testing frameworks. To verify the integrity of all mathematical implementations:

```bash
cd math_explorer
cargo test
```

This runs unit tests for everything from Prime Number generation to NeRF rendering logic.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for our style guide and process.

**The Golden Rule:** If you add code, you must add documentation and tests.

## 📄 License

This project is open-source. See the [LICENSE](LICENSE) file for details.
