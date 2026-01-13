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

    /// Calculates the slope of the frequency chirp ($S$).
    ///
    /// $$ S = \frac{B}{T_c} $$
    pub fn slope(&self) -> f64 {
        self.bandwidth / self.chirp_time
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

    /// Calculates the maximum unambiguous velocity ($v_{max}$).
    ///
    /// Limits the detectable velocity before Doppler aliasing occurs.
    ///
    /// $$ v_{max} = \frac{\lambda}{4 T_c} $$
    pub fn max_unambiguous_velocity(&self) -> f64 {
        self.wavelength() / (4.0 * self.chirp_time)
    }

    /// Estimates target range ($R$) from the IF beat frequency ($f_b$).
    ///
    /// $$ f_b = \frac{S \cdot 2R}{c} \implies R = \frac{f_b c}{2 S} $$
    ///
    /// # Arguments
    ///
    /// * `beat_frequency` - The measured beat frequency in Hz.
    pub fn range_from_beat_frequency(&self, beat_frequency: f64) -> f64 {
        // existing implementation used (beat_frequency * C * chirp_time) / (2.0 * bandwidth)
        // which is equivalent since slope = bandwidth / chirp_time
        (beat_frequency * C) / (2.0 * self.slope())
    }

    /// Calculates the expected beat frequency ($f_b$) for a target at distance $R$.
    ///
    /// $$ f_b = \frac{S \cdot 2R}{c} $$
    ///
    /// # Arguments
    ///
    /// * `range` - Distance to target in meters.
    pub fn beat_frequency(&self, range: f64) -> f64 {
        (self.slope() * 2.0 * range) / C
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

    /// Calculates the instantaneous frequency of the chirp at time $t$.
    ///
    /// $$ f(t) = S t + f_c $$
    ///
    /// # Arguments
    /// * `t` - Time since start of chirp (0 <= t <= chirp_time).
    pub fn chirp_frequency(&self, t: f64) -> f64 {
        self.slope() * t + self.center_frequency
    }
}

/// Calculates the output of the mixer (heterodyne principle).
///
/// $$ x_{out} = \sin((\omega_1 - \omega_2)t + (\phi_1 - \phi_2)) $$
///
/// # Arguments
/// * `w1` - Angular frequency of signal 1 ($\omega_1$).
/// * `w2` - Angular frequency of signal 2 ($\omega_2$).
/// * `phi1` - Initial phase of signal 1 ($\phi_1$).
/// * `phi2` - Initial phase of signal 2 ($\phi_2$).
/// * `t` - Time.
pub fn mixer_output(w1: f64, w2: f64, phi1: f64, phi2: f64, t: f64) -> f64 {
    ((w1 - w2) * t + (phi1 - phi2)).sin()
}

/// Calculates the dielectric phase delay.
///
/// Models the additional phase shift induced by passing through a dielectric material (e.g., immobilization mask).
///
/// $$ \Delta \phi = \frac{4\pi d}{\lambda} (\sqrt{\epsilon_r} - 1) $$
///
/// # Arguments
/// * `d` - Material thickness in meters.
/// * `lambda` - Operating wavelength in meters.
/// * `epsilon_r` - Relative dielectric constant of the material.
pub fn dielectric_phase_delay(d: f64, lambda: f64, epsilon_r: f64) -> f64 {
    (4.0 * std::f64::consts::PI * d / lambda) * (epsilon_r.sqrt() - 1.0)
}
