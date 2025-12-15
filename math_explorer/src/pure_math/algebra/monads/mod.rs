//! # Monads
//!
//! This module implements the mathematical concept of Monads, specifically mapping
//! the abstract definitions of Functor ($T$), Unit ($\eta$), and Multiplication ($\mu$)
//! onto concrete data structures: **Lists** and **Maybe** (Option).
//!
//! ## Summary Table: The "Monoid" View
//!
//! | Monad | The "Set" (Functor) | Identity $e$ ($\eta$) | Binary Op ($\mu$) |
//! | :--- | :--- | :--- | :--- |
//! | **List** | All possible lists | Create generic singleton list `[x]` | Concatenate/Flatten `[[x]]` $\to$ `[x]` |
//! | **Maybe** | $X \cup \{\text{Null}\}$ | Wrap in "Success" tag `Just x` | Collapse tags `Just (Just x)` $\to$ `Just x` |

pub mod list;
pub mod maybe;
