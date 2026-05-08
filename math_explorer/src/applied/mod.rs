#![doc = include_str!("README.md")]

pub mod algorithms;
pub mod battery_degradation;
pub mod cannibalism;
pub mod clinical_trials;
pub mod engineering;
pub mod favoritism;
pub mod freesurfer;
pub mod game_theory;
pub mod grpo;
pub mod isosurface;
pub mod lorahub;
pub mod pharmacokinetics;
pub mod win_ratio;

// pub mod generative_turbulence;

pub use engineering::error as engineering_error;
