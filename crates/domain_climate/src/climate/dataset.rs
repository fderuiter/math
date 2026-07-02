//! Dummy dataset generation for global temperature anomalies.

use rand::Rng;

/// Generates synthetic global temperature anomaly data.
/// Returns a vector of [year, anomaly] pairs.
#[verified_engine::verified]
pub fn get_temperature_anomalies() -> Vec<[f64; 2]> {
    let mut rng = oxidize_core::rng::OxidizeRng::default();
    get_temperature_anomalies_with_rng(&mut rng)
}

/// Generates synthetic global temperature anomaly data using an injected RNG.
/// Returns a vector of [year, anomaly] pairs.
#[verified_engine::verified]
pub fn get_temperature_anomalies_with_rng<R: Rng + ?Sized>(rng: &mut R) -> Vec<[f64; 2]> {
    let mut time_series = Vec::new();
    let mut current_anomaly = -0.5;

    for year in 1850..=2020 {
        let noise: f64 = rng.gen_range(-0.15..0.15);
        let trend = if year < 1950 { 0.002 } else { 0.015 };
        current_anomaly += trend + noise;
        time_series.push([year as f64, current_anomaly]);
    }

    time_series
}

/// Generates synthetic global CO2 concentration data (in ppm) and projections.
/// Historical data up to 2020, then projection up to 2100 based on the `reduction_scenario` factor.
/// `reduction_scenario` from 0.0 (business as usual) to 1.0 (aggressive reduction).
/// Returns a tuple of (historical_data, projected_data).
#[verified_engine::verified]
pub fn get_co2_projections(reduction_scenario: f64) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let mut rng = oxidize_core::rng::OxidizeRng::default();
    get_co2_projections_with_rng(&mut rng, reduction_scenario)
}

/// Generates synthetic global CO2 concentration data and projections using an injected RNG.
#[verified_engine::verified]
pub fn get_co2_projections_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    reduction_scenario: f64,
) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let mut historical = Vec::new();
    let mut projected = Vec::new();

    let mut current_co2 = 280.0; // Pre-industrial baseline roughly

    // Historical 1850 to 2020
    for year in 1850..=2020 {
        let noise: f64 = rng.gen_range(-0.5..0.5);
        let increase = if year < 1950 { 0.1 } else { 1.5 };
        current_co2 += increase + noise;
        historical.push([year as f64, current_co2]);
    }

    // Projected 2021 to 2100
    // If reduction_scenario == 0.0, it keeps increasing by ~2.5 ppm/year
    // If reduction_scenario == 1.0, increase goes to negative
    let mut current_increase = 1.5;

    for year in 2021..=2100 {
        let noise: f64 = rng.gen_range(-0.5..0.5);
        let target_increase = 2.5 - (3.5 * reduction_scenario);

        current_increase += (target_increase - current_increase) * 0.05;
        current_co2 += current_increase + noise;

        projected.push([year as f64, current_co2]);
    }

    (historical, projected)
}
