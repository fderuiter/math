//! Radar Physics for FMCW Sensors (TI IWR6843).
//!
//! This module implements the fundamental physics equations for Frequency Modulated Continuous Wave (FMCW)
//! radar systems, specifically focusing on range resolution, Doppler velocity estimation, and signal properties.
//!
//! The TI IWR6843 is a mmWave sensor operating in the 60-64 GHz band.

use std::f64::consts::PI;

/// The speed of light in vacuum in meters per second (m/s).
pub const C: f64 = 299_792_458.0;

/// Configuration for the FMCW Radar system.
#[derive(Debug, Clone, Copy)]
pub struct FmcwConfig {
    /// Sweep bandwidth in Hz ($B$).
    pub bandwidth: f64,
    /// Center frequency in Hz ($f_c$).
    pub center_frequency: f64,
    /// Chirp duration (separation time between chirps) in seconds ($T_c$).
    pub chirp_time: f64,
    /// Slope of the frequency chirp ($S$) in Hz/s.
    pub slope: f64,
}

impl FmcwConfig {
    /// Creates a new configuration for the TI IWR6843 with default values.
    ///
    /// - Bandwidth: 4 GHz
    /// - Center Frequency: 60 GHz
    /// - Chirp Time: 50 microseconds
    /// - Slope: 80 MHz/us (typical, calculated from bandwidth/time roughly if not specified,
    ///   here we assume a specific slope or user must provide it).
    ///   Let's assume S = B / T_c for default active time.
    pub fn iwr6843_default() -> Self {
        let bandwidth = 4.0e9;
        let chirp_time = 50.0e-6;
        Self {
            bandwidth,
            center_frequency: 60.0e9,
            chirp_time,
            slope: bandwidth / chirp_time, // Approximation for effective slope
        }
    }

    /// Calculates the fundamental range resolution ($\Delta R$).
    ///
    /// The ability to resolve two distinct points in depth is determined by the sweep bandwidth $B$.
    ///
    /// $$ \Delta R = \frac{c}{2B} $$
    pub fn range_resolution(&self) -> f64 {
        C / (2.0 * self.bandwidth)
    }

    /// Calculates the signal wavelength ($\lambda$).
    ///
    /// $$ \lambda = \frac{c}{f_c} $$
    pub fn wavelength(&self) -> f64 {
        C / self.center_frequency
    }

    /// Calculates the Maximum Unambiguous Velocity ($v_{max}$).
    ///
    /// $$ v_{max} = \frac{\lambda}{4 T_c} $$
    ///
    /// The maximum velocity detectable without aliasing, determined by the chirp repetition time.
    pub fn max_unambiguous_velocity(&self) -> f64 {
        self.wavelength() / (4.0 * self.chirp_time)
    }

    /// Calculates the Beat Frequency ($f_b$) for a target at distance $R$.
    ///
    /// $$ f_b = \frac{S \cdot 2R}{c} $$
    ///
    /// # Arguments
    /// * `range` - Distance to target ($R$) in meters.
    pub fn beat_frequency(&self, range: f64) -> f64 {
        (self.slope * 2.0 * range) / C
    }

    /// Estimates target range ($R$) from the measured beat frequency ($f_b$).
    ///
    /// Inverse of `beat_frequency`.
    ///
    /// $$ R = \frac{f_b c}{2 S} $$
    pub fn range_from_beat_frequency(&self, beat_frequency: f64) -> f64 {
        (beat_frequency * C) / (2.0 * self.slope)
    }

    /// Estimates radial velocity ($v$) from the phase shift ($\Delta \phi$) between chirps.
    ///
    /// $$ \Delta \phi = \frac{4\pi v T_c}{\lambda} \implies v = \frac{\Delta \phi \lambda}{4\pi T_c} $$
    pub fn velocity_from_phase(&self, phase_shift: f64) -> f64 {
        let lambda = self.wavelength();
        (phase_shift * lambda) / (4.0 * PI * self.chirp_time)
    }

    /// Estimates physical displacement ($d$) from the phase change ($\Delta \phi$).
    ///
    /// $$ d = \frac{\lambda \Delta \phi}{4\pi} $$
    pub fn displacement_from_phase(&self, phase_shift: f64) -> f64 {
        let lambda = self.wavelength();
        (phase_shift * lambda) / (4.0 * PI)
    }

    /// Calculates the required angular separation ($\Delta\theta_{req}$) to resolve two targets.
    ///
    /// Uses the small-angle approximation:
    /// $$ \Delta\theta_{\text{req}} \approx \arctan\left(\frac{d_{\text{TAA}}}{R}\right) $$
    ///
    /// # Arguments
    /// * `vertical_separation` - Distance between targets ($d_{TAA}$).
    /// * `range` - Distance from sensor ($R$).
    pub fn required_angular_separation(vertical_separation: f64, range: f64) -> f64 {
        (vertical_separation / range).atan()
    }

    /// Calculates the theoretical Angle Resolution ($\theta_{res}$) for a MIMO array.
    ///
    /// $$ \theta_{res} = \frac{\lambda}{N_{RX} l \cos(\theta)} $$
    ///
    /// # Arguments
    /// * `num_rx` - Number of virtual receive antennas (or array aperture elements).
    /// * `antenna_spacing` - Distance between elements ($l$).
    /// * `angle` - Angle off-boresight ($\theta$).
    pub fn angle_resolution(&self, num_rx: usize, antenna_spacing: f64, angle: f64) -> f64 {
        self.wavelength() / (num_rx as f64 * antenna_spacing * angle.cos())
    }

    /// Estimates Angle of Arrival ($\theta$) from phase difference ($\Delta \phi$) across antennas.
    ///
    /// $$ \theta = \sin^{-1} \left( \frac{\lambda \Delta \phi}{2 \pi l} \right) $$
    ///
    /// # Arguments
    /// * `phase_diff` - Phase difference between adjacent antennas.
    /// * `antenna_spacing` - Distance between antennas ($l$).
    pub fn angle_of_arrival(&self, phase_diff: f64, antenna_spacing: f64) -> f64 {
        let arg = (self.wavelength() * phase_diff) / (2.0 * PI * antenna_spacing);
        // Clamp for safety against numerical noise > 1.0
        arg.clamp(-1.0, 1.0).asin()
    }
}

/// Calculates the instantaneous frequency of a Linear Frequency Modulated (LFM) chirp.
///
/// $$ f(t) = S t + f_c $$
pub fn chirp_frequency(t: f64, slope: f64, start_freq: f64) -> f64 {
    slope * t + start_freq
}

/// Models the output of the mixer (Heterodyne principle).
///
/// $$ x_{out} = \sin((\omega_1 - \omega_2)t + (\phi_1 - \phi_2)) $$
///
/// This represents the beat signal composed of the difference frequencies.
pub fn mixer_output(
    t: f64,
    omega1: f64,
    omega2: f64,
    phi1: f64,
    phi2: f64,
) -> f64 {
    ((omega1 - omega2) * t + (phi1 - phi2)).sin()
}
