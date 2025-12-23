//! # Deep Learning Theory
//!
//! This module implements the mathematical foundations of Deep Learning **from first principles**.
//! It avoids high-level abstractions (like `torch.nn.Linear`) to expose the raw machinery of training.
//!
//! ## 🎓 The Curriculum
//!
//! The submodules map directly to the four pillars of Deep Learning math:
//!
//! 1.  [`linear_algebra`]: **The Structure**. Defines `DenseLayer`, `Vector`, and matrix multiplication logic (`z = Wx + b`).
//! 2.  [`probability`]: **The Goal**. Defines the objective function using Maximum Likelihood Estimation (e.g., Softmax).
//! 3.  [`calculus`]: **The Engine**. Implements the Chain Rule manually to derive gradients (Backpropagation).
//! 4.  [`optimization`]: **The Driver**. Updates weights using Stochastic Gradient Descent (SGD).
//! 5.  [`cycle`]: **The Loop**. Puts it all together in a `TrainingLoop`.
//!
//! ## 💡 Why this exists?
//!
//! Modern frameworks hide the complexity of auto-differentiation and optimizer states.
//! This module reveals the "magic" by explicitly calculating derivatives (`dL/dw`, `dL/db`) and passing them around.

pub mod linear_algebra;
pub mod calculus;
pub mod probability;
pub mod optimization;
pub mod cycle;
