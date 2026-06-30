/// Defines the core behavior for a pharmacokinetic model.
pub trait PharmacokineticModel {
    /// Computes the concentration at time `t`.
    ///
    /// # Arguments
    /// * `t` - The time at which to calculate the concentration.
    ///
    /// # Returns
    /// The concentration at time `t`.
    #[verified_engine::verified]
    fn concentration(&self, t: f64) -> f64;
}
