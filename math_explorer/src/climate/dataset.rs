//! Dummy dataset generation for global temperature anomalies.

use rand::Rng;

/// Generates synthetic global temperature anomaly data.
/// Returns a vector of [year, anomaly] pairs.
pub fn get_temperature_anomalies() -> Vec<[f64; 2]> {
    let mut time_series = Vec::new();
    let mut rng = rand::thread_rng();
    let mut current_anomaly = -0.5;

    for year in 1850..=2020 {
        let noise: f64 = rng.gen_range(-0.15..0.15);
        let trend = if year < 1950 { 0.002 } else { 0.015 };
        current_anomaly += trend + noise;
        time_series.push([year as f64, current_anomaly]);
    }

    time_series
}
