## 2025-05-18 - [Decoupling CERA from Concrete Autoencoder]
**Violation:** **Dependency Inversion Principle (DIP)** and **Open/Closed Principle (OCP)**. The `Cera` class (High-level module) depends directly on the concrete `Autoencoder` struct (Low-level module). Additionally, `CeraTrainer` violates **Law of Demeter** and **Encapsulation** by manually iterating over the autoencoder's internal layers to apply weight updates.
**Remedy:** **Extract Interface** (`AutoencoderModel`) and **Dependency Injection**. I will introduce an `AutoencoderModel` trait that exposes `forward`, `encode`, and `update_weights`. `Cera` will hold a `Box<dyn AutoencoderModel>`. The weight update logic will be moved from the trainer to the model, respecting encapsulation.

## 2024-10-14 - Decoupled Gradient Estimation from Marching Cubes
**Violation:** SRP/OCP. The `extract_isosurface` function was a God Function mixing grid iteration, optimization logic, and gradient calculation strategies (Central Differences), making it impossible to extend with new estimators (e.g., Sobel) without modifying core logic.
**Remedy:** Extracted `GradientEstimator` Strategy interface and `CentralDifferenceEstimator`. Refactored `MarchingCubes` to use dependency injection for the estimator, preserving the optimized fast-path via a specialized unsafe trait method.

## 2025-05-19 - [Decoupling Enzyme Kinetics from Concrete Implementation]
**Violation:** **Open/Closed Principle (OCP)** and **Dependency Inversion Principle (DIP)**. The `EnzymeReaction` struct was a concrete implementation of Michaelis-Menten kinetics, preventing the extension of the system to support other kinetic models (e.g., Hill, Inhibition) without modifying the existing struct.
**Remedy:** **Extract Interface** (`KineticsModel`). Renamed `EnzymeReaction` to `MichaelisMenten` and implemented the trait. Added `HillKinetics` as a new implementation to demonstrate extensibility. Maintained backward compatibility via a deprecated type alias.

## 2025-05-24 - [Decoupling Transformer Layers from Concrete Implementations]
**Violation:** **Dependency Inversion Principle (DIP)**. `EncoderLayer` and `DecoderLayer` directly depended on concrete structs (`MultiHeadAttention`, `FeedForward`, `LayerNorm`), making it impossible to swap implementations (e.g., for Sparse Attention or RMSNorm) or mock them for testing.
**Remedy:** **Dependency Injection with Generics**. Extracted `AttentionMechanism`, `FeedForwardNetwork`, and `NormalizationLayer` traits. Refactored `EncoderLayer`, `DecoderLayer`, and `Transformer` to accept these dependencies via generics, defaulting to the original implementations for full backward compatibility.

## 2025-05-25 - [Decoupling Auction Mechanism from Simulation Harness]
**Violation:** **Open/Closed Principle (OCP)**. The `MechanismDesign` struct was a "Utility Class" anti-pattern that hardcoded logic for Optimal Auctions (Myerson's Lemma) within static methods, making it impossible to simulate other auction types (e.g., Second-Price, First-Price) without modifying the core simulation loop.
**Remedy:** **Strategy Pattern**. Extracted `AuctionMechanism` trait. Refactored `MechanismDesign` to delegate to a new `OptimalAuction` strategy. Implemented `SecondPriceAuction` to prove extensibility. The simulation harness now depends on the abstraction, not the concrete implementation.
