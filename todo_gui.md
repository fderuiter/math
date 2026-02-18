# GUI Integration Roadmap for Math Explorer

This document outlines the roadmap for integrating the various modules of the `math_explorer` library into the graphical user interface (`math_explorer_gui`). The goal is to provide interactive visualizations and controls for each domain, enabling users to explore mathematical concepts intuitively.

## 1. Physics

### 1.1 MRI (Magnetic Resonance Imaging) - **[Implemented]**
*   **Features:**
    *   [x] Bloch Simulator (magnetization vector evolution).
    *   [x] Controls for $T_1$, $T_2$, $\vec{B}$-field, and $M_0$.
    *   [x] Real-time plotting of $M_x, M_y, M_z$.

### 1.2 Quantum Mechanics
*   **Module:** `physics::quantum`
*   **Features:**
    *   [x] **Schrödinger Equation Solver:** 1D potential well visualizer (particle in a box, harmonic oscillator).
    *   [x] **Wavefunction Evolution:** Animation of $|\psi|^2$ over time.
    *   [x] **Clebsch-Gordan Coefficients:** Interactive calculator for coupling angular momenta.
    *   [x] **Spin Dynamics:** Bloch sphere visualization for qubits.

### 1.3 Fluid Dynamics
*   **Module:** `physics::fluid_dynamics`
*   **Features:**
    *   [x] **Flow Visualization:** 2D heatmap or vector field plot of velocity/pressure.
    *   [ ] **Turbulence Simulation:** Parameter controls for Reynolds number.
    *   [ ] **Lattice Boltzmann Demo:** Interactive obstacle placement in a flow stream.

### 1.4 Chaos Theory
*   **Module:** `physics::chaos`
*   **Features:**
    *   [ ] **Attractor Plotter:** 3D interactive plot for Lorenz, Rossler, and other strange attractors.
    *   [ ] **Bifurcation Diagrams:** Logistic map explorer with zoom capabilities.
    *   [ ] **Fractal Generator:** Mandelbrot/Julia set viewer with pan/zoom.

### 1.5 Solid State Physics
*   **Module:** `physics::solid_state`
*   **Features:**
    *   [ ] **Crystal Lattice Viewer:** 3D visualization of unit cells (FCC, BCC, SC).
    *   [ ] **Band Structure:** Plot E-k diagrams for simple potentials.
    *   [ ] **Ising Model:** 2D grid simulation of spin flips and phase transitions (temperature slider).

### 1.6 Medical Physics
*   **Module:** `physics::medical`
*   **Features:**
    *   [ ] **Dose Calculation:** 2D heatmap of radiation dose distribution.
    *   [ ] **Beam Profiling:** Interactive depth-dose curves.

---

## 2. Biology

### 2.1 Neuroscience
*   **Module:** `biology::neuroscience`
*   **Features:**
    *   [ ] **Hodgkin-Huxley Model:** Voltage trace plotter with controls for ion channel conductances ($g_{Na}, g_{K}, g_{L}$).
    *   [ ] **Spike Train Analysis:** Raster plots and ISI histograms.
    *   [ ] **Neural Network Viz:** Graph view of connected neurons and firing activity.

### 2.2 Epidemiology
*   **Module:** `epidemiology`
*   **Features:**
    *   [ ] **SIR/SEIR Models:** Time-series plots of Susceptible, Infected, Recovered populations.
    *   [ ] **Parameter Sliders:** Adjust transmission rate ($\beta$) and recovery rate ($\gamma$).
    *   [ ] **Network Propagation:** Graph visualization of disease spread through a population.

### 2.3 Evolutionary Game Theory
*   **Module:** `applied::game_theory::evolutionary`
*   **Features:**
    *   [ ] **Replicator Dynamics:** Phase plane plots for Hawk-Dove or Rock-Paper-Scissors games.
    *   [ ] **Population Bar Charts:** Real-time updating of strategy proportions.

### 2.4 Morphogenesis
*   **Module:** `biology::morphogenesis`
*   **Features:**
    *   [ ] **Turing Patterns:** 2D grid visualization of Reaction-Diffusion systems (e.g., Gray-Scott).
    *   [ ] **Pattern Gallery:** Presets for spots, stripes, and labyrinths.

---

## 3. Artificial Intelligence (AI)

### 3.1 Deep Learning Theory
*   **Module:** `ai::deep_learning_theory`
*   **Features:**
    *   [ ] **Loss Landscape:** 3D surface plot of loss functions.
    *   [ ] **Training Monitor:** Real-time curves for training/validation loss and accuracy.
    *   [ ] **Activation Functions:** Interactive plotter for ReLU, Sigmoid, Tanh, GELU, etc.

### 3.2 Reinforcement Learning
*   **Module:** `ai::reinforcement_learning`
*   **Features:**
    *   [ ] **Grid World:** Agent navigation visualization.
    *   [ ] **Q-Table Inspector:** Heatmap of Q-values.
    *   [ ] **Reward Plots:** Cumulative reward over episodes.

### 3.3 Transformers
*   **Module:** `ai::transformer`
*   **Features:**
    *   [ ] **Attention Maps:** Heatmap visualization of self-attention weights.
    *   [ ] **Tokenization:** Text input field showing token breakdown and embeddings.

---

## 4. Applied Mathematics

### 4.1 Battery Degradation
*   **Module:** `applied::battery_degradation`
*   **Features:**
    *   [ ] **Capacity Fade:** Plot capacity vs. cycle number based on depth of discharge (DoD) and temperature.
    *   [ ] **Lifetime Estimator:** Calculator for expected battery life under specific usage profiles.

### 4.2 Clinical Trials
*   **Module:** `applied::clinical_trials`
*   **Features:**
    *   [ ] **Survival Curves:** Kaplan-Meier plot generator.
    *   [ ] **Sample Size Calculator:** Form inputs for $\alpha$, $\beta$, and effect size.
    *   [ ] **Randomization:** Interactive subject allocation tool.

### 4.3 Favoritism (Satirical)
*   **Module:** `applied::favoritism`
*   **Features:**
    *   [ ] **Family Leaderboard:** Dynamic ranking of family members based on "favoritism scores".
    *   [ ] **Input Form:** Sliders for "Gift Value", "Call Frequency", "Compliments".

### 4.4 Financial Math (Kelly Criterion)
*   **Module:** `pure_math::statistics::kelly`
*   **Features:**
    *   [ ] **Bankroll Growth:** Simulation of wealth over multiple bets using Kelly vs. fractional Kelly.
    *   [ ] **Bet Size Calculator:** Input fields for odds and probability of winning.

---

## 5. Pure Mathematics

### 5.1 Analysis & Calculus
*   **Module:** `pure_math::analysis`
*   **Features:**
    *   [ ] **ODE/PDE Solvers:** Generic solver interface where users can type equations (parsed or selected) and see solutions.
    *   [ ] **Riemann Integration:** Visualization of area under curves with adjustable partition size.
    *   [ ] **Complex Mapping:** Conformal map visualizer (z-plane to w-plane grid transformation).

### 5.2 Number Theory
*   **Module:** `pure_math::number_theory`
*   **Features:**
    *   [ ] **Prime Spiral:** Ulam spiral visualization.
    *   [ ] **Factorization Tool:** Large number factorization and primality testing.
    *   [ ] **Partition Function:** Calculator and visualization of integer partitions (Ferrers diagrams).

### 5.3 Graph Theory
*   **Module:** `pure_math::graph_theory`
*   **Features:**
    *   [ ] **Graph Editor:** Add/remove nodes and edges interactively.
    *   [ ] **Algorithm visualizer:** Step-by-step animation of Dijkstra, BFS, DFS.
    *   [ ] **Network Metrics:** Calculate centrality, diameter, and clustering coefficients.

### 5.4 Geometry & Topology
*   **Module:** `pure_math::differential_geometry` / `pure_math::topology`
*   **Features:**
    *   [ ] **Surface Viewer:** 3D parametric surface plotter (torus, sphere, klein bottle).
    *   [ ] **Curvature Heatmap:** Color-code surfaces by Gaussian or Mean curvature.
    *   [ ] **Simplicial Complexes:** Visualization of Vietoris-Rips complexes from point clouds (TDA).

---

## 6. Climate & Environment

### 6.1 Climate Modeling
*   **Module:** `climate`
*   **Features:**
    *   [ ] **Temperature Anomalies:** Time-series visualization of global temperature data.
    *   [ ] **CERA Model:** Interactive inputs for the Coupled Energy-Resource-Atmosphere model.
    *   [ ] **CO2 Projections:** Scenario sliders for emissions reduction.

---

## Implementation Strategy

1.  **Modular UI:** Use `egui::Tab` or a sidebar navigation to switch between domains.
2.  **Shared Components:** Create reusable widgets for common tasks (TimeStepper controls, 2D/3D Plotters, Matrix viewers).
3.  **Performance:** Use separate threads for heavy computations (Physics/AI simulations) to keep the UI responsive.
4.  **Interactivity:** Prioritize real-time feedback. Changing a slider should immediately update the visualization where possible.
