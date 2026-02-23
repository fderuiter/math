//! Preset configurations for Turing patterns.

/// Defines standard presets for the Schnakenberg reaction-diffusion system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternPreset {
    /// Classic spots (leopard-like).
    Spots,
    /// Stripe patterns (zebra-like).
    Stripes,
    /// Labyrinthine/Maze-like structures.
    Labyrinth,
    /// Chaotic/Unstable evolution.
    Chaos,
    /// User-defined parameters.
    Custom,
}

impl PatternPreset {
    /// Returns the display name of the preset.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Spots => "Spots",
            Self::Stripes => "Stripes",
            Self::Labyrinth => "Labyrinth",
            Self::Chaos => "Chaos",
            Self::Custom => "Custom",
        }
    }

    /// Returns the parameters (a, b, d_u, d_v) for the preset.
    /// Returns None for Custom.
    pub fn params(&self) -> Option<(f64, f64, f64, f64)> {
        match self {
            // Schnakenberg typical spot parameters
            // High inhibitor diffusion (d_v >> d_u) promotes spots.
            Self::Spots => Some((0.1, 0.9, 1.0, 100.0)),

            // Lowering d_v often transitions to stripes or labyrinths.
            // Also adjusting 'a' slightly helps.
            Self::Stripes => Some((0.1, 0.9, 1.0, 10.0)),

            // Specific kinetic balance for labyrinths.
            Self::Labyrinth => Some((0.12, 0.88, 1.0, 20.0)),

            // Unstable regime.
            Self::Chaos => Some((0.02, 0.98, 1.0, 50.0)),

            Self::Custom => None,
        }
    }

    /// Iterates over all presets except Custom.
    pub fn all() -> [Self; 4] {
        [Self::Spots, Self::Stripes, Self::Labyrinth, Self::Chaos]
    }
}
