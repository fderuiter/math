//! # Applied Mathematics
//!
//! This module serves as a bridge between abstract mathematical theory and concrete, real-world problems.
//! By applying concepts from calculus, statistics, linear algebra, and game theory, we solve domain-specific
//! challenges ranging from medical imaging to satirical social modeling.
//!
//! ## 🏥 Biology & Medicine
//!
//! *   **[Clinical Trials](clinical_trials)**: Design and analysis tools, including sample size calculation and hypothesis testing.
//! *   **[Pharmacokinetics](pharmacokinetics)**: Models for drug absorption, distribution, metabolism, and excretion (ADME).
//! *   **[FreeSurfer](freesurfer)**: Tools for analyzing neuroimaging data, compatible with the FreeSurfer suite.
//! *   **[Cannibalism](cannibalism)**: Population dynamics models focusing on intraspecific predation.
//!
//! ## ⚡ Physics & Engineering
//!
//! *   **[Battery Degradation](battery_degradation)**: Modeling capacity fade and health over charge cycles.
//! *   **[Isosurface](isosurface)**: Algorithms for extracting 3D surfaces from volumetric data (e.g., Marching Cubes).
//!
//! ## 🤖 Artificial Intelligence
//!
//! *   **[LoRA Hub](lorahub)**: Logic for merging Low-Rank Adaptation (LoRA) weights for LLMs.
//! *   **[GRPO](grpo)**: Group Relative Policy Optimization components for Reinforcement Learning.
//!
//! ## 🎭 Social Science (Satire)
//!
//! *   **[Favoritism](favoritism)**: A "rigorous" mathematical framework for quantifying parental affection.
//!
//! ## 🧮 General Algorithms
//!
//! *   **[Game Theory](game_theory)**: Strategic decision-making models, including Mean Field Games.
//! *   **[Win Ratio](win_ratio)**: Statistical methods for comparing composite endpoints.
//! *   **[Algorithms](algorithms)**: Fundamental structures including sorting strategies.
//!
//! > **Note:** Some modules like `generative_turbulence` are currently experimental or deprecated and thus not exposed publicly.

pub mod battery_degradation;
pub mod cannibalism;
pub mod favoritism;
pub mod freesurfer;
pub mod win_ratio;
pub mod pharmacokinetics;
pub mod lorahub;
pub mod game_theory;
pub mod grpo;
pub mod isosurface;
pub mod clinical_trials;
pub mod algorithms;
