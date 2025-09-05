//! # Favoritism
//!
//! This module contains the implementation of the satirical favoritism formula.

use nalgebra::{DMatrix, DVector};
use quadrature::clenshaw_curtis;
use rand::Rng;

#[derive(Debug)]
pub struct FavoritismInputs {
    pub t: f64,
    pub g_emotional: f64,
    pub g_practical: f64,
    pub f_initial: f64,
    pub birth_order_weight: f64,
    pub major_life_events: f64,
    pub helped_during_crisis: bool,
    pub active_on_social_media: bool,
    pub decay_constant: f64,
    pub time_since_last_contact: f64,
    pub intelligence: f64,
    pub emotional_sensitivity: f64,
    pub wealth: f64,
    pub talent: f64,
    pub w_i: f64,
    pub w_es: f64,
    pub w_w: f64,
    pub w_t: f64,
    pub sibling_distances: Vec<f64>,
    pub compliments: DVector<f64>,
    pub compliment_weights: DVector<f64>,
    pub x_0: f64,
}

impl Default for FavoritismInputs {
    fn default() -> Self {
        Self {
            t: 365.0,
            g_emotional: 5.0,
            g_practical: 2.0,
            f_initial: 7.0,
            birth_order_weight: 1.2,
            major_life_events: 3.0,
            helped_during_crisis: true,
            active_on_social_media: true,
            decay_constant: 0.05,
            time_since_last_contact: 7.0,
            intelligence: 7.0,
            emotional_sensitivity: 6.0,
            wealth: 9.0,
            talent: 8.0,
            w_i: 1.2,
            w_es: 1.5,
            w_w: 1.1,
            w_t: 1.3,
            sibling_distances: vec![100.0, 50.0, 10.0],
            compliments: DVector::from_vec(vec![10.0, 5.0, 8.0]),
            compliment_weights: DVector::from_vec(vec![1.0, 0.5, 0.75]),
            x_0: 20.0,
        }
    }
}

pub fn calculate_favoritism_score(inputs: &FavoritismInputs) -> f64 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.9..1.1);

    let proximity_integral = clenshaw_curtis::integrate(|_t| 1.0 / inputs.x_0, 0.0, inputs.t, 1e-9).integral;

    let emotional_support_integral = clenshaw_curtis::integrate(
        |_t| {
            clenshaw_curtis::integrate(|_x| 8.0, 0.0, 1.0, 1e-9).integral
        },
        0.0,
        inputs.t,
        1e-9,
    )
    .integral;

    let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![inputs.g_emotional, inputs.g_practical]));
    let gift_matrix_determinant = gift_matrix.determinant();

    let compliment_score = inputs.compliments.dot(&inputs.compliment_weights);

    let frequency_term = (1.0 + inputs.f_initial).ln();

    let personality_score = inputs.w_i * inputs.intelligence
        + inputs.w_es * inputs.emotional_sensitivity
        + inputs.w_w * inputs.wealth
        + inputs.w_t * inputs.talent;

    let h = if inputs.helped_during_crisis { 1.5 } else { 1.0 };
    let s = if inputs.active_on_social_media { 1.3 } else { 1.0 };
    let d = (-inputs.decay_constant * inputs.time_since_last_contact).exp();

    let sibling_proximity_integral = clenshaw_curtis::integrate(
        |_t| {
            inputs
                .sibling_distances
                .iter()
                .map(|distance| 1.0 / distance)
                .sum()
        },
        0.0,
        inputs.t,
        1e-9,
    )
    .integral;

    let numerator = proximity_integral
        * emotional_support_integral
        * gift_matrix_determinant
        * compliment_score
        * frequency_term
        * personality_score
        * inputs.birth_order_weight
        * inputs.major_life_events
        * h
        * s
        * d
        * r;

    let denominator = sibling_proximity_integral;

    numerator / denominator
}
