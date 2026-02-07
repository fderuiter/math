# Curator's Log - Documentation Decision Records (DDR)

## 2025-05-15 - Visualizing the Domain Ecosystem
**Gap:** The root README lists domains but lacks a visual map of how they relate or what they contain, making the project feel like a disconnected list of files.
**Strategy:** Introduce a Mermaid.js diagram in the "Features" section to visualize the project hierarchy.
**Outcome:** Users can instantly grasp the scope of the library without reading the table of contents.

## 2025-05-15 - Elevating Biology
**Gap:** The "Biology" module is a core feature but is completely absent from the "Deep Dive" sections in the READMEs, violating the "Show, Don't Tell" principle.
**Strategy:** Add a Hodgkin-Huxley neuron simulation example to both the root and crate READMEs.
**Outcome:** Demonstrates the library's capability in complex biological systems modeling.

## 2025-05-16 - Closing the "What is this?" Gap
**Gap:** `pure_math` lacked a top-level description, and `radar_gating` was a wall of text describing a pipeline.
**Strategy:** Added a summary module doc to `pure_math` and a Mermaid pipeline diagram to `radar_gating`.
**Outcome:** Users can now visualize the data flow in radar processing and understand the scope of the pure math module at a glance.

## 2025-05-17 - Demystifying the AI Black Box
**Gap:** The `ai` module documentation was a dry list of files, failing to convey the "From Scratch" educational philosophy or the relationships between submodules (e.g., how SDS relates to NeRF).
**Strategy:** Overhauled `ai/mod.rs` with a "Deep Learning & AI" header, a Mermaid ecosystem diagram, and a runnable Transformer example. Also added a process diagram to `sds/mod.rs`.
**Outcome:** Users can now visualize the AI learning path and run a Transformer model in seconds.

## 2026-01-17 - Illuminating the Physics Modules
**Gap:** The `physics::chaos` module was messy with implementation plans and lacked a clear visual explanation. The `physics::quantum` module was a complete "blank page" despite being used in the READMEs "Hello World".
**Strategy:**
- Overhauled `physics/chaos/mod.rs` with a "Deterministic Chaos" primer, a Mermaid diagram of the Butterfly Effect, and a runnable Lorenz System example.
- Overhauled `physics/quantum/mod.rs` with a "Time Evolution" workflow diagram and examples for Clebsch-Gordan coupling and Qubit evolution.
**Outcome:** Users can now immediately grasp the core concepts of Chaos and Quantum mechanics without reading source code, and have copy-pasteable examples for both.

## 2025-05-18 - Visualizing High Energy Physics
**Gap:** The `physics/high_energy` module was a collection of disconnected tools (relativity, radiation, fluids) without a unifying narrative or example.
**Strategy:** Added a "Black Hole Observer" Quick Start example that combines General Relativity (gravity) and Special Relativity (motion). Also added a Mermaid diagram showing the interaction of these forces.
**Outcome:** Users can now model a complex physical scenario (orbiting a black hole) immediately.

## 2025-05-18 - Documenting Ghost Modules
**Gap:** The `generative_turbulence` module existed on disk but was excluded from the build ("Ghost Module") with no explanation for the user.
**Strategy:** Added a `README.md` inside the module explaining the `tch` dependency constraint and instructions on how to enable it.
**Outcome:** Converts a confusing "missing" feature into an opt-in power user feature.

## 2025-05-19 - Visualizing Number Theory
**Gap:** The `pure_math/number_theory` module was a "Visual Void" with a generic description and the `partitions` submodule was a "Black Box" of undocumented math.
**Strategy:**
- Overhauled `number_theory/mod.rs` with a Mermaid diagram and clear module breakdown.
- Removed deprecated `is_prime_placeholder` ("Rot Check").
- Added comprehensive docstrings and a "Quick Start" example to `partitions.rs`, explaining the Pushpa and Vasuki functions.
**Outcome:** Users can now navigate the number theory tools and understand how to generate restricted partition coefficients.

## 2026-01-25 - Demystifying Algorithmic Information
**Gap:** The `algorithmic_information` module was a "Visual Void" with opaque submodule names and zero context for the mathematical theory.
**Strategy:** Overhauled `mod.rs` with a primer on Kolmogorov Complexity, a Mermaid diagram, and a runnable Quick Start. Added doc examples to `kolmogorov.rs` and `geometry.rs`.
**Outcome:** Users can now understand the connection between program size and complexity and use the approximation tools without reading the source code.

## 2026-01-26 - Visualizing LoraHub
**Gap:** The `lorahub` module was a "Visual Void" with no explanation of the Strategy Pattern or how to use the ensemble.
**Strategy:** Overhauled `lorahub/mod.rs` with a Mermaid class diagram showing the composition strategies and a runnable Quick Start example for merging LoRA weights.
**Outcome:** Users can now understand the LoraHub architecture and merge weights without reading the source code.

## 2026-02-01 - Demystifying Neuroimaging
**Gap:** The `freesurfer` module was a "Visual Void" with no explanation of the cortical reconstruction pipeline or usage.
**Strategy:** Overhauled `freesurfer/mod.rs` with a Mermaid pipeline diagram and a runnable Quick Start example for cortical thickness calculation.
**Outcome:** Users can now understand the MRI processing pipeline and use the tools for surface analysis.

## 2026-02-15 - Visualizing Fluid Dynamics
**Gap:** The `physics/fluid_dynamics` module is a core domain but has a practically empty `mod.rs` ("Visual Void"), hiding the well-structured Strategy Pattern implementation of conservation laws.
**Strategy:** Overhaul `fluid_dynamics/mod.rs` with a Mermaid diagram showing the relationship between Properties, State, and Momentum Equations, and add a "Quick Start" example using Navier-Stokes.
**Outcome:** Users can now understand how to simulate fluid flow and use the Strategy Pattern for different flow regimes.

## 2026-02-16 - Visualizing Abstract Algebra
**Gap:** The `pure_math/algebra` module contained placeholder code ("Rot") and a sparse description, failing to explain the rich type hierarchy (Groups, Rings, Fields) implemented.
**Strategy:** Remove `placeholder_add`. Add a Mermaid Class Diagram of the algebraic hierarchy and a "Quick Start" example demonstrating Finite Fields (`Fp`) and Polynomial arithmetic.
**Outcome:** Users can now visualize the mathematical structure and immediately start using finite field arithmetic.

## 2026-02-16 - Demystifying GRPO
**Gap:** The `applied/grpo` module was a "Blank Page" with zero context, hiding a complex Reinforcement Learning algorithm.
**Strategy:** Add a Mermaid Flowchart of the optimization loop and a "Quick Start" example for calculating the clipped surrogate objective.
**Outcome:** Users can now understand the flow of Group Relative Policy Optimization and use the objective function components.

## 2026-02-17 - Visualizing Pharmacokinetics
**Gap:** The `applied/pharmacokinetics` module was a "Visual Void" with sparse docs, hiding the powerful Trait-based composition system for ADME modeling.
**Strategy:** Overhauled `pharmacokinetics/mod.rs` with a Mermaid diagram of the model hierarchy and a runnable Quick Start example for simulating drug concentration.
**Outcome:** Users can now understand the relationship between Base and Wrapper models and simulate dosing regimens without reading the source code.

## 2026-02-18 - Visualizing Win Ratio Analysis
**Gap:** The `applied/win_ratio` module was a "Visual Void" with sparse documentation and no explanation of the complex hierarchical comparison logic.
**Strategy:** Overhauled `win_ratio/mod.rs` with a Mermaid flowchart of the comparison decision tree and a runnable Quick Start example. Added detailed docstrings to the `pair_comparison` module.
**Outcome:** Users can now understand the pairwise comparison logic (Death > HF > QoL) and calculate statistics without reading the source code.

## 2026-02-18 - Unearthing Battery Degradation
**Gap:** The `battery_degradation` module was a "Ghost Module" (missing from the root README) and its documentation pointed to deprecated functions.
**Strategy:** Updated `battery_degradation/mod.rs` to promote the modern `PowerLawModel` API with a clear example. Added the module to the root README "Features" table and diagram.
**Outcome:** The module is now discoverable and users are guided to use the correct, type-safe API.

## 2026-02-20 - Visualizing Gaussian Splatting
**Gap:** The `gaussian_splatting` module was a "Visual Void" with opaque submodules (projection, rendering) and no high-level explanation of the rasterization pipeline.
**Strategy:** Overhaul `gaussian_splatting/mod.rs` with a Mermaid diagram of the Forward Pass (3D->2D->Image) and Adaptive Density Control, plus a runnable "Quick Start" example.
**Outcome:** Users can now visualize the rendering flow and understand how 3D Gaussians are projected and blended.

## 2026-02-21 - Visualizing Algorithms
**Gap:** The `applied/algorithms` module is a "Visual Void" with minimal documentation, failing to explain the purpose or usage of key algorithms like the Kalman Filter.
**Strategy:** Overhaul `algorithms/mod.rs` with a "Taxonomy" Mermaid diagram, a detailed description, and a runnable "Quick Start" example for 1D Kalman tracking.
**Outcome:** Users can now visualize the estimation loop and implement state tracking immediately.
