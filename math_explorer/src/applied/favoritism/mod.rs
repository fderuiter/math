//! # Favoritism
//!
//! This module contains the implementation of the satirical favoritism formula.

pub mod favorite_child;
pub mod scoring;
pub mod types;

pub use scoring::calculate_favoritism_score;
pub use types::{
    FavoritismInputs,
    TimeParams,
    GiftParams,
    ContactParams,
    PersonalityParams,
    SocialParams,
    ComplimentParams,
    FamilyParams,
};
