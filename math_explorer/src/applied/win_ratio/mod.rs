//! # Win Ratio Analysis
//!
//! The **Win Ratio** is a statistical method used in clinical trials to compare two groups (e.g., Treatment vs. Control)
//! using composite endpoints. It is particularly useful when endpoints have a clear hierarchy of importance
//! (e.g., **Death** is more important than **Hospitalization**, which is more important than **Quality of Life**).
//!
//! ## The Problem
//! Traditional methods like Cox Proportional Hazards often treat the first event as the only event, ignoring subsequent
//! events or their severity. The Win Ratio method allows for a prioritized comparison of all patient pairs.
//!
//! ## How It Works (Hierarchical Comparison)
//!
//! For every pair of patients (one from Treatment, one from Control), we compare their outcomes starting from the
//! most severe endpoint.
//!
//! ```mermaid
//! graph TD
//!     Start[Compare Pair: Treated vs Control] --> Q1{Did the Control patient<br>die first?}
//!     Q1 -- Yes --> Win[Win for Treatment]
//!     Q1 -- No --> Q2{Did the Treated patient<br>die first?}
//!     Q2 -- Yes --> Loss[Loss for Treatment]
//!     Q2 -- No --> Q3{Did the Control patient<br>have a HF Event first?}
//!     Q3 -- Yes --> Win
//!     Q3 -- No --> Q4{Did the Treated patient<br>have a HF Event first?}
//!     Q4 -- Yes --> Loss
//!     Q4 -- No --> Tie[Tie / Move to Next Endpoint]
//!
//!     style Win fill:#aaffaa,stroke:#333
//!     style Loss fill:#ffaaaa,stroke:#333
//!     style Tie fill:#ffffaa,stroke:#333
//! ```
//!
//! The **Win Ratio** is calculated as:
//! $$ \text{Win Ratio} = \frac{N_{wins}}{N_{losses}} $$
//!
//! ## Quick Start
//!
//! ```rust
//! use math_explorer::applied::win_ratio::pair_comparison::{unmatched_pairs, calculate_statistics};
//!
//! // Define outcomes for two groups.
//! // Hierarchy: [Days to Death (Lower is worse), Days to Hospitalization (Lower is worse)]
//! // Note: In this simple example, we use "Time to Event".
//! // If a patient didn't have an event, use a max duration (e.g., 999).
//!
//! // Group A (Treatment): Lived longer, fewer hospitalizations
//! let group_treatment = vec![
//!     vec![999.0, 999.0], // Survived, No Hosp
//!     vec![999.0, 500.0], // Survived, Hosp at day 500
//!     vec![700.0, 200.0], // Died at day 700
//! ];
//!
//! // Group B (Control): Died earlier
//! let group_control = vec![
//!     vec![100.0, 50.0],  // Died at day 100
//!     vec![200.0, 100.0], // Died at day 200
//!     vec![999.0, 300.0], // Survived, Hosp at day 300
//! ];
//!
//! // 1. Compare every patient in Treatment vs every patient in Control
//! // Note: Comparison logic depends on direction.
//! // Here we assume Higher value = Better Outcome (Longer survival).
//! let (wins, losses) = unmatched_pairs(&group_treatment, &group_control);
//!
//! // 2. Calculate Statistics
//! if let Some(stats) = calculate_statistics(wins, losses) {
//!     println!("Win Ratio: {:.2}", stats.win_ratio);
//!     println!("95% CI: [{:.2}, {:.2}]", stats.ci_low, stats.ci_high);
//!     println!("P-Value: {:.4}", stats.p_value);
//! }
//! ```
//!
//! ## Modules
//!
//! - [`pair_comparison`]: Core logic for matched and unmatched pair comparisons.
//! - [`sample_win_ratio`]: Tools for sample-based estimation.
//! - [`probability_win_ratio`]: Theoretical probability calculations.
//! - [`simulation`]: Monte Carlo simulations for power analysis.
//! - [`bmi`]: Utility for calculating Body Mass Index (BMI), a common covariate in cardiovascular trials.

pub mod bmi;
pub mod pair_comparison;
pub mod probability_win_ratio;
pub mod sample_win_ratio;
pub mod simulation;
