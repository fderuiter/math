use nalgebra::Vector3;

/// Defines the type of crystal system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrystalSystem {
    SimpleCubic,
    BodyCenteredCubic,
    FaceCenteredCubic,
}

/// Represents a unit cell with lattice vectors and atomic positions.
#[derive(Debug, Clone)]
pub struct UnitCell {
    /// The three lattice vectors defining the unit cell.
    pub lattice_vectors: [Vector3<f64>; 3],
    /// The positions of atoms within the unit cell.
    pub atomic_positions: Vec<Vector3<f64>>,
}

impl CrystalSystem {
    /// Generates the unit cell for the given lattice constant.
    pub fn generate(&self, a: f64) -> UnitCell {
        // Lattice vectors for a cubic system are always aligned with axes
        let lattice_vectors = [
            Vector3::new(a, 0.0, 0.0),
            Vector3::new(0.0, a, 0.0),
            Vector3::new(0.0, 0.0, a),
        ];

        let atomic_positions = match self {
            CrystalSystem::SimpleCubic => vec![Vector3::new(0.0, 0.0, 0.0)],
            CrystalSystem::BodyCenteredCubic => vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.5 * a, 0.5 * a, 0.5 * a),
            ],
            CrystalSystem::FaceCenteredCubic => vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.5 * a, 0.5 * a, 0.0),
                Vector3::new(0.5 * a, 0.0, 0.5 * a),
                Vector3::new(0.0, 0.5 * a, 0.5 * a),
            ],
        };

        UnitCell {
            lattice_vectors,
            atomic_positions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cubic() {
        let system = CrystalSystem::SimpleCubic;
        let cell = system.generate(1.0);
        assert_eq!(cell.atomic_positions.len(), 1);
        assert_eq!(cell.atomic_positions[0], Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_bcc() {
        let system = CrystalSystem::BodyCenteredCubic;
        let cell = system.generate(2.0);
        assert_eq!(cell.atomic_positions.len(), 2);
        assert_eq!(cell.atomic_positions[1], Vector3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_fcc() {
        let system = CrystalSystem::FaceCenteredCubic;
        let cell = system.generate(2.0);
        assert_eq!(cell.atomic_positions.len(), 4);
        assert_eq!(cell.atomic_positions[1], Vector3::new(1.0, 1.0, 0.0));
    }
}
