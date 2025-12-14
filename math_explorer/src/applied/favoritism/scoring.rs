use super::types::FavoritismInputs;
use nalgebra::{DMatrix, DVector};
use quadrature::clenshaw_curtis;
use rand::Rng;

/// Calculates the favoritism score based on the provided inputs.
///
/// The formula combines integrals of proximity and emotional support, gift value,
/// compliments, personality traits, and various other factors, scaled by a
/// random perturbation `r`.
///
/// # Arguments
///
/// * `inputs` - A reference to a `FavoritismInputs` struct containing all necessary parameters.
///
/// # Returns
///
/// A `f64` representing the calculated favoritism score. Higher is better.
pub fn calculate_favoritism_score(inputs: &FavoritismInputs) -> f64 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.9..1.1);

    let proximity_integral = clenshaw_curtis::integrate(|_t| 1.0 / inputs.time.x_0, 0.0, inputs.time.t, 1e-9).integral;

    let emotional_support_integral = clenshaw_curtis::integrate(
        |_t| {
            clenshaw_curtis::integrate(|_x| 8.0, 0.0, 1.0, 1e-9).integral
        },
        0.0,
        inputs.time.t,
        1e-9,
    )
    .integral;

    let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![inputs.gifts.g_emotional, inputs.gifts.g_practical]));
    let gift_matrix_determinant = gift_matrix.determinant();

    let compliment_score = inputs.compliments.compliments.dot(&inputs.compliments.compliment_weights);

    let frequency_term = (1.0 + inputs.contact.f_initial).ln();

    let personality_score = inputs.personality.w_i * inputs.personality.intelligence
        + inputs.personality.w_es * inputs.personality.emotional_sensitivity
        + inputs.personality.w_w * inputs.personality.wealth
        + inputs.personality.w_t * inputs.personality.talent;

    let h = if inputs.social.helped_during_crisis { 1.5 } else { 1.0 };
    let s = if inputs.social.active_on_social_media { 1.3 } else { 1.0 };
    let d = (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp();

    let sibling_proximity_integral = clenshaw_curtis::integrate(
        |_t| {
            inputs
                .family
                .sibling_distances
                .iter()
                .map(|distance| 1.0 / distance)
                .sum()
        },
        0.0,
        inputs.time.t,
        1e-9,
    )
    .integral;

    let numerator = proximity_integral
        * emotional_support_integral
        * gift_matrix_determinant
        * compliment_score
        * frequency_term
        * personality_score
        * inputs.social.birth_order_weight
        * inputs.social.major_life_events
        * h
        * s
        * d
        * r;

    let denominator = sibling_proximity_integral;

    numerator / denominator
}
