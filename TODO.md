# OxidizeMath Architecture Refactoring Roadmap

## Phase 1: Safe Structural Migration
- [ ] Create workspace directories: `mkdir crates apps`
- [ ] Perform Git-aware moves to preserve commit history:
  - [ ] `git mv math_explorer crates/`
  - [ ] `git mv math_explorer_gui apps/`
- [ ] Update root `Cargo.toml` workspace members to point to `"crates/math_explorer"` and `"apps/math_explorer_gui"`.
- [ ] Run `cargo check --workspace` to ensure zero compilation path errors.

## Phase 2: Establishing `oxidize_core` (The WASM-Safe Engine Contract)
- [ ] Initialize core library: `cd crates && cargo new oxidize_core --lib`
- [ ] Add `"crates/oxidize_core"` to root `Cargo.toml` workspace members.
- [ ] Define agnostic simulation traits in `crates/oxidize_core/src/lib.rs`:
  - [ ] `trait ModelConfig`
  - [ ] `trait ModelState`
  - [ ] `trait SimulationModel` (with `initialize`, `step`, and `get_state` methods).
- [ ] Add standard agnostic dependencies (`thiserror`, `serde` with `derive`) to `oxidize_core/Cargo.toml`.
- [ ] Link core to facade: Add `oxidize_core = { path = "../oxidize_core" }` to `crates/math_explorer/Cargo.toml`.

## Phase 3: Purging the Facade (`math_explorer` lib.rs) and Fixing Tests
- [ ] Move `test_number_theory_is_prime` out of `lib.rs` and into `src/pure_math/number_theory/primes.rs`.
- [ ] Move `test_clebsch_gordan_*` tests out of `lib.rs` into `tests/test_quantum.rs`.
- [ ] Move `test_lorahub_functions` out of `lib.rs` into `src/applied/lorahub/ensemble.rs`.
- [ ] Move `test_find_favorite_child` out of `lib.rs` into `tests/test_favoritism.rs`.
- [ ] Clean `crates/math_explorer/src/lib.rs`:
  - [ ] Remove all test blocks.
  - [ ] Re-export core: `pub use oxidize_core::*;`
  - [ ] Maintain only `pub mod` declarations and `pub use` statements.
- [ ] Run `cargo test --workspace` and verify 100% pass rate.

## Phase 4: Domain Sub-Crate Extraction (Physical Split)
- [ ] Extract Pure Math: Create `crates/oxidize_math` and move `math_explorer/src/pure_math/*` into it.
- [ ] Extract Physics: Create `crates/oxidize_physics` and move `math_explorer/src/physics/*` into it.
- [ ] Extract AI: Create `crates/oxidize_ai` and move `math_explorer/src/ai/*` into it.
- [ ] Extract Biology: Create `crates/oxidize_biology` and move `math_explorer/src/biology/*` into it.
- [ ] Extract Applied/Other: Create `crates/oxidize_applied` and move `math_explorer/src/applied/*` into it.
- [ ] Update all internal `use crate::...` paths in the extracted crates.
- [ ] For each domain crate, declare a dependency on `oxidize_core` in their respective `Cargo.toml`.
- [ ] Refactor `crates/math_explorer/Cargo.toml` to declare all new crates as `optional = true` dependencies.
- [ ] Feature-gate the facade (`math_explorer`) to conditionally export domains (e.g., `[features] ai = ["oxidize_ai"]`).

## Phase 5: Dependency Isolation & Optimization
- [ ] `oxidize_physics`: Make `rustfft`, `wigner-symbols`, `num-complex` optional and feature-gated.
- [ ] `oxidize_math`: Make `rug`, `petgraph` optional and feature-gated.
- [ ] Replace standard OS-dependent threading/randomness with WASM-safe alternatives where necessary:
  - [ ] Add `js` feature flag to `getrandom` dependency for targeting WASM.
  - [ ] Feature-gate any `rayon` parallelization (disable for WASM, enable for native).

## Phase 6: WebAssembly (WASM) GUI Integration (`math_explorer_gui`)
- [ ] Configure `apps/math_explorer_gui` to compile to target `wasm32-unknown-unknown`.
- [ ] Set up Web interface using `eframe` (egui) and bundle with `Trunk` (`index.html`).
- [ ] Implement the "No-I/O" Rule: Ensure GUI models only accept configurations via buffer arrays or strings (e.g., `serde` over `&[u8]`).
- [ ] Migrate the first model (e.g., simple ODE or Lorenz Chaos) to strictly implement the `SimulationModel` trait from `oxidize_core`.
- [ ] Plug the migrated model into the web GUI to verify standard execution loops (init -> step -> render_state) without blocking the main thread.
- [ ] (Future enhancement) Set up Web Workers to offload heavy `step()` computations from the egui rendering thread.
