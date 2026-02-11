//! # Mechanism Design
//!
//! This module provides tools for designing and analyzing economic mechanisms, particularly auctions.
//! It supports:
//! - **Optimal Auctions**: Revenue-maximizing mechanisms (Myerson).
//! - **Standard Auctions**: Second-price (Vickrey) auctions.
//! - **Simulation**: Monte Carlo revenue estimation.
//!
//! ## Example
//!
//! ```
//! use math_explorer::applied::game_theory::mechanism_design::auction::{OptimalAuction, AuctionMechanism};
//! use statrs::distribution::Uniform;
//!
//! let dist = Uniform::new(0.0, 100.0).unwrap();
//!
//! // Create an optimal auction for this distribution
//! let auction = OptimalAuction::new(&dist, 0.0, 100.0);
//! println!("Optimal Reserve Price: {:.2}", auction.reserve_price);
//!
//! // Simulate revenue with 5 bidders
//! let revenue = auction.expected_revenue(&dist, 5, 1000);
//! println!("Expected Revenue: {:.2}", revenue);
//! ```

pub mod traits;
pub mod auction;
pub mod legacy;

pub use traits::*;
pub use auction::*;
#[allow(deprecated)]
pub use legacy::MechanismDesign;
