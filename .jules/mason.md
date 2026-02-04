## 2025-05-18 - [Decoupling CERA from Concrete Autoencoder]
**Violation:** **Dependency Inversion Principle (DIP)** and **Open/Closed Principle (OCP)**. The `Cera` class (High-level module) depends directly on the concrete `Autoencoder` struct (Low-level module). Additionally, `CeraTrainer` violates **Law of Demeter** and **Encapsulation** by manually iterating over the autoencoder's internal layers to apply weight updates.
**Remedy:** **Extract Interface** (`AutoencoderModel`) and **Dependency Injection**. I will introduce an `AutoencoderModel` trait that exposes `forward`, `encode`, and `update_weights`. `Cera` will hold a `Box<dyn AutoencoderModel>`. The weight update logic will be moved from the trainer to the model, respecting encapsulation.

## 2024-10-14 - Decoupled Gradient Estimation from Marching Cubes
**Violation:** SRP/OCP. The `extract_isosurface` function was a God Function mixing grid iteration, optimization logic, and gradient calculation strategies (Central Differences), making it impossible to extend with new estimators (e.g., Sobel) without modifying core logic.
**Remedy:** Extracted `GradientEstimator` Strategy interface and `CentralDifferenceEstimator`. Refactored `MarchingCubes` to use dependency injection for the estimator, preserving the optimized fast-path via a specialized unsafe trait method.
