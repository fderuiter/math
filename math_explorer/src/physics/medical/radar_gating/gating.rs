//! Gating Logic for Radiation Beam Control.
//!
//! Implements the decision layer for respiratory gating, using a Schmidt Trigger
//! and latency compensation to control the LINAC beam-off switch.

/// Configuration and State for the Gating Logic.
#[derive(Debug, Clone)]
pub struct GatingLogic {
    /// Amplitude threshold for gating (e.g., peak exhalation level).
    pub threshold: f64,
    /// Hysteresis margin to prevent chattering (e.g., 2 mm).
    pub hysteresis: f64,
    /// Hardware latency of the beam-off switch in seconds (e.g., 0.060).
    pub system_latency: f64,
    /// Current state of the beam (true = ON, false = OFF).
    pub is_beam_on: bool,
}

impl GatingLogic {
    /// Creates a new Gating Logic instance.
    pub fn new(threshold: f64, hysteresis: f64, system_latency: f64) -> Self {
        Self {
            threshold,
            hysteresis,
            system_latency,
            is_beam_on: false,
        }
    }

    /// Evaluates whether the beam should be ON or OFF based on the current tracker state.
    ///
    /// Uses a **Schmidt Trigger** logic:
    /// - If the signal drops below `threshold - hysteresis` (deep breath/stable), turn Beam ON.
    /// - If the signal rises above `threshold + hysteresis` (motion/exhale), turn Beam OFF.
    ///
    /// (Note: The direction "below" or "above" depends on whether gating is amplitude-based
    /// deep inspiration breath hold (DIBH) or free breathing.
    /// Here we assume a generic "lower is stable/target" logic, or we can invert it.
    /// Let's assume the user wants to gate *within* a window or below a threshold.
    ///
    /// Given the prompt: "The binary beam trigger uses a Schmidt Trigger... near the threshold."
    /// And the latency compensation equation: $A_{pred} = A_{filt} + \dot{A} \cdot L_{lat}$.
    ///
    /// We will implement "Beam ON when $A_{pred} < T$".
    /// With hysteresis:
    /// - Turn ON if $A_{pred} < T - H$.
    /// - Turn OFF if $A_{pred} > T + H$.
    /// This assumes "low amplitude" is the target state (e.g. baseline).
    pub fn evaluate(&mut self, amplitude: f64, velocity: f64) -> bool {
        // Latency Compensation
        // Predict the amplitude at the time the beam actually switches.
        let predicted_amplitude = amplitude + velocity * self.system_latency;

        if self.is_beam_on {
            // Currently ON. Check if we need to switch OFF.
            // Upper threshold.
            if predicted_amplitude > self.threshold + self.hysteresis {
                self.is_beam_on = false;
            }
        } else {
            // Currently OFF. Check if we need to switch ON.
            // Lower threshold.
            if predicted_amplitude < self.threshold - self.hysteresis {
                self.is_beam_on = true;
            }
        }

        self.is_beam_on
    }
}
