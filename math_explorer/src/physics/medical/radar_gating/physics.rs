//! Radar Physics for FMCW Sensors (TI IWR6843).
//!
//! This module implements the fundamental physics equations for Frequency Modulated Continuous Wave (FMCW)
//! radar systems, specifically focusing on range resolution and Doppler velocity estimation.
//!
//! The TI IWR6843 is a mmWave sensor operating in the 60-64 GHz band.

/// The speed of light in vacuum in meters per second (m/s).
pub const C: f64 = 299_792_458.0;

/// Configuration for the FMCW Radar system.
#[derive(Debug, Clone, Copy)]
pub struct FmcwConfig {
    /// Sweep bandwidth in Hz (e.g., 4.0e9 for 4 GHz).
    pub bandwidth: f64,
    /// Center frequency in Hz (e.g., 60.0e9 for 60 GHz).
    pub center_frequency: f64,
    /// Chirp duration (separation time between chirps) in seconds.
    pub chirp_time: f64,
}

impl FmcwConfig {
    /// Creates a new configuration for the TI IWR6843 with default values.
    ///
    /// - Bandwidth: 4 GHz
    /// - Frequency: 60 GHz
    /// - Chirp Time: 50 microseconds
    pub fn iwr6843_default() -> Self {
        Self {
            bandwidth: 4.0e9,
            center_frequency: 60.0e9,
            chirp_time: 50.0e-6,
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

    /// Estimates radial velocity ($v$) from the phase shift ($\Delta \phi$) between chirps.
    ///
    /// Doppler processing uses the phase change across a sequence of chirps separated by time $T_c$.
    ///
    /// $$ \Delta \phi = \frac{4\pi v T_c}{\lambda} \implies v = \frac{\Delta \phi \lambda}{4\pi T_c} $$
    ///
    /// # Arguments
    ///
    /// * `phase_shift` - The phase shift in radians.
    pub fn velocity_from_phase(&self, phase_shift: f64) -> f64 {
        let lambda = self.wavelength();
        (phase_shift * lambda) / (4.0 * std::f64::consts::PI * self.chirp_time)
    }

    /// Estimates target range ($R$) from the IF beat frequency ($\hat{f}_{FFT}$).
    ///
    /// Equation (1) from the Bressler et al. framework:
    /// $$ \hat{f}_{FFT} = \frac{2 B R}{c T} \implies R = \frac{\hat{f}_{FFT} c T}{2 B} $$
    ///
    /// # Arguments
    ///
    /// * `beat_frequency` - The measured beat frequency in Hz.
    pub fn range_from_beat_frequency(&self, beat_frequency: f64) -> f64 {
        (beat_frequency * C * self.chirp_time) / (2.0 * self.bandwidth)
    }

    /// Estimates physical displacement ($d$) from the phase change ($\Delta \phi$).
    ///
    /// Equation (5) from the Bressler et al. framework:
    /// $$ d = \frac{\lambda \Delta \phi}{4\pi} $$
    ///
    /// Note: This is equivalent to velocity * chirp_time, but expressed directly as displacement.
    ///
    /// # Arguments
    ///
    /// * `phase_shift` - The phase shift in radians.
    pub fn displacement_from_phase(&self, phase_shift: f64) -> f64 {
        let lambda = self.wavelength();
        (phase_shift * lambda) / (4.0 * std::f64::consts::PI)
    }
}
