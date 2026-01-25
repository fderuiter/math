# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.
## 2026-01-08 - Decomposed Standard Model God File
**Violation:** Single Responsibility Principle (SRP). The `standard_model.rs` file was a 'God File' containing unrelated physics domains (Gauge, Higgs, Flavor, QCD, Neutrinos) in a single module.
**Remedy:** Module Extraction. Split the file into a directory `standard_model/` with separate files for each domain (`gauge.rs`, `higgs.rs`, etc.) and re-exported them via `mod.rs`. This improves cohesion and navigability.

## 2026-01-20 - Deterministic Simulation via RNG Injection
**Violation:** Dependency Inversion Principle (DIP). The `simulate_optimal_revenue` (Mechanism Design) and `calculate_favoritism_score` (Favoritism) functions hardcoded `rand::thread_rng()`, making them non-deterministic and untestable without mocking.
**Remedy:** Method Injection. Extracted core logic into `*_with_rng` methods accepting `&mut R: Rng`. The original functions now serve as convenience wrappers. This enables deterministic testing with seeded RNGs.

## 2026-01-14 - Decoupling Numerical Integration
**Violation:** Dependency Inversion Principle (DIP). The `calculate_favoritism_score` (Favoritism) and `calculate_win_probability` (Win Ratio) functions hardcoded `quadrature::clenshaw_curtis`, creating a tight coupling to a specific integration implementation and preventing strategy substitution.
**Remedy:** Strategy Pattern. Created `Integrator` trait in `pure_math/analysis/integration.rs` and implemented `ClenshawCurtis` and `Trapezoidal` strategies. Refactored high-level modules to accept `&impl Integrator` via dependency injection.

## 2026-05-22 - Decoupling Statistical Distributions
**Violation:** Open/Closed Principle (OCP). The `occupancy_probability` function switched on a `ParticleType` enum, requiring modification to the function to support new statistical distributions.
**Remedy:** Strategy Pattern. Extracted `StatisticalDistribution` trait and implemented `FermiDirac`, `BoseEinstein`, and `MaxwellBoltzmann` strategies. Refactored `occupancy_probability` to delegate to these strategies.

## 2026-06-15 - Decoupling Favoritism Scoring Factors
**Violation:** Single Responsibility Principle (SRP) and Open/Closed Principle (OCP). The `calculate_favoritism_score_full` function was a monolithic block handling Proximity, Gifts, Personality, and Social factors, making it impossible to extend without modification.
**Remedy:** Composition and Strategy Pattern. Created `UnifiedFavoritismModel` which composes individual `ScoringStrategy` components (`ProximityStrategy`, `GiftStrategy`, etc.). This allows extending the model with new factors while maintaining the core equation structure.

## 2026-06-25 - Decoupling LoraHub Algorithms
**Violation:** Open/Closed Principle (OCP). The `LoraEnsemble` hardcoded `combine` (weighted sum) and `evaluate_objective` (L1 Regularization), preventing extension for advanced merging techniques like SLERP or TIES.
**Remedy:** Strategy Pattern. Extracted `CombinationStrategy` and `ObjectiveStrategy` traits. Implemented `LinearCombinationStrategy` and `L1RegularizationStrategy`. Refactored `LoraEnsemble` to use these strategies via dependency injection.

## 2026-01-25 - Decoupling RL Exploration
**Violation:** Open/Closed Principle (OCP) and Single Responsibility Principle (SRP). `TabularQAgent` hardcoded Epsilon-Greedy logic and managed exploration parameters (`epsilon`), preventing extension (e.g., Softmax) and violating separation of concerns.
**Remedy:** Strategy Pattern. Extracted `ExplorationStrategy` trait and `EpsilonGreedy` implementation. Refactored `TabularQAgent` to delegate action selection to the injected strategy.
