# Comprehensive OxidizeMath Architecture Refactoring Roadmap

This document outlines the step-by-step, granular process for migrating the OxidizeMath monolithic repository into a modular, WASM-compatible Workspace with a Facade pattern.

## Phase 1: Workspace Foundation & Safe Migration
*Goal: Establish the physical folder boundaries without breaking Git history or compilation paths.*

- [x] **1.1 Directory Setup**
  - [x] Run `mkdir crates apps` at the repository root.
- [ ] **1.2 Git-Aware File Moves** (CRITICAL: Use `git mv` to preserve history)
  - [ ] Run `git mv math_explorer crates/`
  - [ ] Run `git mv math_explorer_gui apps/`
- [ ] **1.3 Root Workspace Configuration**
  - [ ] Create/Update `Cargo.toml` at the repository root.
  - [ ] Add `[workspace]` section.
  - [ ] Add `members = ["crates/math_explorer", "apps/math_explorer_gui"]`.
  - [ ] Set `resolver = "2"`.
- [ ] **1.4 Verification**
  - [ ] Run `cargo check --workspace`. Ensure zero errors before proceeding.

## Phase 2: The Core Engine Contract (`oxidize_core`)
*Goal: Create the dependency-free law of the land that allows the GUI to interact with mathematical models agnostically.*

- [ ] **2.1 Initialize Crate**
  - [ ] Run `cd crates && cargo new oxidize_core --lib`.
  - [ ] Add `"crates/oxidize_core"` to the root `Cargo.toml` workspace members.
- [ ] **2.2 Configure Dependencies** (In `crates/oxidize_core/Cargo.toml`)
  - [ ] Add `serde = { version = "1.0", features = ["derive"] }` (Required for GUI to save/load configs).
  - [ ] Add `thiserror = "1.0"` (For standardizing engine errors).
- [ ] **2.3 Define Architectural Traits** (In `crates/oxidize_core/src/lib.rs`)
  - [ ] Define `pub trait ModelConfig: Clone + serde::Serialize + serde::Deserialize<'static> {}`
  - [ ] Define `pub trait ModelState: Clone {}`
  - [ ] Define `pub trait SimulationModel`.
  - [ ] Add method: `fn initialize(config: Self::Config) -> Result<Self, Self::Error> where Self: Sized;`
  - [ ] Add method: `fn step(&mut self) -> Result<(), Self::Error>;`
  - [ ] Add method: `fn get_state(&self) -> Self::State;`
- [ ] **2.4 Link to Facade**
  - [ ] In `crates/math_explorer/Cargo.toml`, add `oxidize_core = { path = "../oxidize_core" }`.

## Phase 3: Facade Purge & Test Relocation
*Goal: Strip `math_explorer/src/lib.rs` down to a pure router. Tests belong in their respective modules or the `tests/` directory.*

- [ ] **3.1 Relocate Pure Math Tests**
  - [ ] Move `test_number_theory_is_prime` to `crates/math_explorer/src/pure_math/number_theory/primes.rs`.
- [ ] **3.2 Relocate Quantum Tests**
  - [ ] Move `test_clebsch_gordan_*` tests to `crates/math_explorer/tests/test_quantum.rs`.
- [ ] **3.3 Relocate Applied Math Tests**
  - [ ] Move `test_lorahub_functions` to `crates/math_explorer/src/applied/lorahub/ensemble.rs`.
  - [ ] Move `test_find_favorite_child` to `crates/math_explorer/tests/test_favoritism.rs`.
- [ ] **3.4 Clean `lib.rs`**
  - [ ] Remove the `#[cfg(test)]` block entirely from `crates/math_explorer/src/lib.rs`.
  - [ ] Add `pub use oxidize_core::*;` to the top of the file.
- [ ] **3.5 Verification**
  - [ ] Run `cargo test --workspace`. Ensure all relocated tests still pass.

## Phase 4: Domain Sub-Crate Extraction (The Physical Split)
*Goal: Isolate mathematical domains into their own crates to prevent dependency hell.*

### 4.1 Extract `oxidize_math` (Pure Math)
- [ ] Create `crates/oxidize_math` (`cargo new oxidize_math --lib`).
- [ ] Add to root workspace members.
- [ ] `git mv crates/math_explorer/src/pure_math/* crates/oxidize_math/src/`
- [ ] Move `rug` and `petgraph` dependencies from `math_explorer` to `oxidize_math/Cargo.toml`.
- [ ] Add `oxidize_core` dependency.
- [ ] Fix internal `use crate::` paths.

### 4.2 Extract `oxidize_physics`
- [ ] Create `crates/oxidize_physics`. Add to root workspace.
- [ ] `git mv crates/math_explorer/src/physics/* crates/oxidize_physics/src/`
- [ ] Move `rustfft`, `wigner-symbols`, `num-complex` to `oxidize_physics/Cargo.toml`.
- [ ] Add `oxidize_core` dependency.
- [ ] Fix internal paths.

### 4.3 Extract `oxidize_ai`
- [ ] Create `crates/oxidize_ai`. Add to root workspace.
- [ ] `git mv crates/math_explorer/src/ai/* crates/oxidize_ai/src/`
- [ ] Move AI-specific dependencies (`nalgebra`, etc.) to `oxidize_ai/Cargo.toml`.
- [ ] Add `oxidize_core` dependency.
- [ ] Fix internal paths.

### 4.4 Extract Biology, Climate, Epidemiology, and Applied
- [ ] Create `crates/oxidize_biology` & move `src/biology/*`.
- [ ] Create `crates/oxidize_climate` & move `src/climate/*`.
- [ ] Create `crates/oxidize_epidemiology` & move `src/epidemiology/*`.
- [ ] Create `crates/oxidize_applied` & move `src/applied/*`.
- [ ] Add all to root workspace, add `oxidize_core` dependency, and fix internal paths.

## Phase 5: The Facade Router Setup (`math_explorer`)
*Goal: Re-assemble the engine for end-users using Cargo features.*

- [ ] **5.1 Update Dependencies** (In `math_explorer/Cargo.toml`)
  - [ ] Add all `oxidize_*` crates as `optional = true` dependencies using `{ path = "../oxidize_..." }`.
- [ ] **5.2 Define Feature Flags**
  - [ ] `[features]`
  - [ ] `default = []`
  - [ ] `physics = ["oxidize_physics"]`
  - [ ] `ai = ["oxidize_ai"]`
  - [ ] `math = ["oxidize_math"]`
  - [ ] `full = ["physics", "ai", "math", ...]`
- [ ] **5.3 Update `lib.rs`**
  - [ ] Add conditional compilation blocks:
    ```rust
    #[cfg(feature = "physics")]
    pub use oxidize_physics as physics;
    ```
- [ ] **5.4 Verification**
  - [ ] Run `cargo check -p math_explorer --features full`.

## Phase 6: WASM Optimization & Threading Hardening
*Goal: Ensure the mathematical models can compile to the browser's main thread.*

- [ ] **6.1 Eliminate I/O Panics**
  - [ ] Audit models for `std::fs` usage. Replace with buffer/string injection via `ModelConfig`.
- [ ] **6.2 The `rand` Gotcha**
  - [ ] For any crate using `rand`, update `Cargo.toml`:
    ```toml
    [target.'cfg(target_arch = "wasm32")'.dependencies]
    getrandom = { version = "0.2", features = ["js"] }
    ```
- [ ] **6.3 Feature-Gate Threading (`rayon`)**
  - [ ] Audit crates for `rayon` usage.
  - [ ] Wrap parallel iterators (`par_iter`) in standard iterators (`iter`) when targeting WASM or when a `parallel` feature is disabled.

## Phase 7: Web GUI Integration (`math_explorer_gui`)
*Goal: Connect the frontend to the WASM-compiled core engine.*

- [ ] **7.1 Web Build Setup**
  - [ ] Ensure `math_explorer_gui` relies on `eframe` (egui).
  - [ ] Create `index.html` in `apps/math_explorer_gui/`.
  - [ ] Install Trunk (`cargo install trunk`).
- [ ] **7.2 Link the Engine**
  - [ ] Add `math_explorer` to the GUI's `Cargo.toml` with specific features enabled (e.g., `features = ["physics", "math"]`).
- [ ] **7.3 Implement First Trait**
  - [ ] Choose a simple model (e.g., Lorenz Attractor in `oxidize_physics`).
  - [ ] Implement `ModelConfig`, `ModelState`, and `SimulationModel` for it.
- [ ] **7.4 GUI Rendering Loop**
  - [ ] In the egui update loop, call `model.step()` and `model.get_state()`.
  - [ ] Render the state to the screen.
  - [ ] Verify 60FPS execution in the browser via `trunk serve`.

## Phase 8: Final Polish & CI/CD
- [x] Update `.github/workflows/ci.yml` to run tests across the workspace (`cargo test --workspace --all-features`).
- [x] Add a CI step to verify WASM compilation: `cargo check --workspace --target wasm32-unknown-unknown`.
- [x] Run `cargo fmt --all`.
- [x] Run `cargo clippy --workspace --all-features -- -D warnings`.
