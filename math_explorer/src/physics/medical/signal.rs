//! Medical Signal Processing.
//!
//! Utilities for signal analysis in medical physics context (e.g. beam gating, ultrasound).

/// Models a Dirac Delta Composite Function for beam pulses.
///
/// $$ g(x) = \sum_{n=0}^{(t_{off} - t_{on})/\Delta t} \delta(x - x_n) $$
///
/// In practice, discrete modeling represents this as a sequence of pulses.
/// This function returns the number of pulses in the window.
///
/// # Arguments
///
/// * `t_on` ($t_{on}$) - Start time.
/// * `t_off` ($t_{off}$) - End time.
/// * `delta_t` ($\Delta t$) - Pulse interval.
///
/// # Returns
///
/// * `usize` - Number of pulses.
pub fn dirac_pulse_count(t_on: f64, t_off: f64, delta_t: f64) -> usize {
    if delta_t <= 0.0 || t_off < t_on {
        return 0;
    }
    ((t_off - t_on) / delta_t).floor() as usize + 1
}

/// Calculates the Signal-Front Delay.
///
/// $$ t_{\text{delay}} = \lim_{\omega \to \infty} (\frac{\phi(\omega)}{\omega}) $$
///
/// Represents the delay of the wavefront (signal front) in a dispersive medium.
///
/// # Arguments
///
/// * `phase_slope_high_freq` - The limit of $\phi(\omega) / \omega$ as $\omega \to \infty$.
///
/// # Returns
///
/// * `f64` - The delay time.
pub fn signal_front_delay(phase_slope_high_freq: f64) -> f64 {
    phase_slope_high_freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirac_pulse_count() {
        // [0, 10], dt=2 -> 0, 2, 4, 6, 8, 10 -> 6 pulses
        assert_eq!(dirac_pulse_count(0.0, 10.0, 2.0), 6);
        // [0, 9], dt=2 -> 0, 2, 4, 6, 8 -> 5 pulses
        assert_eq!(dirac_pulse_count(0.0, 9.0, 2.0), 5);
        // Invalid
        assert_eq!(dirac_pulse_count(10.0, 0.0, 1.0), 0);
        assert_eq!(dirac_pulse_count(0.0, 10.0, 0.0), 0);
    }

    #[test]
    fn test_signal_front_delay() {
        assert_eq!(signal_front_delay(5.0), 5.0);
    }
}
