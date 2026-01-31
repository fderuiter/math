#[cfg(test)]
mod tests {
    use crate::physics::fluid_dynamics::{
        analysis::{bernoulli_constant, reynolds_number, shear_stress},
        conservation::{
            continuity_divergence, material_derivative_scalar, navier_stokes_time_derivative,
        },
        regimes::{FlatPlateClassifier, FlowClassifier, FlowRegime, PipeFlowClassifier},
        types::{FlowState, FluidProperties},
    };
    use nalgebra::{Matrix3, Vector3};

    #[test]
    fn test_fluid_properties() {
        let water = FluidProperties::water();
        assert!((water.density - 998.2).abs() < 1e-6);
        assert!((water.dynamic_viscosity - 1.002e-3).abs() < 1e-6);

        let nu = water.kinematic_viscosity();
        assert!((nu - 1.002e-3 / 998.2).abs() < 1e-9);
    }

    #[test]
    fn test_material_derivative() {
        let velocity = Vector3::new(1.0, 2.0, 3.0);
        let gradient = Vector3::new(0.1, 0.2, 0.3);
        let local_change = 0.5;

        // D/Dt = 0.5 + 1*0.1 + 2*0.2 + 3*0.3 = 0.5 + 0.1 + 0.4 + 0.9 = 1.9
        let result = material_derivative_scalar(local_change, velocity, gradient);
        assert!((result - 1.9).abs() < 1e-9);
    }

    #[test]
    fn test_reynolds_number_and_strategies() {
        let props = FluidProperties::new(1000.0, 0.001); // Water-like
        let u = 2.0;
        let l = 0.5;

        // Re = 1000 * 2 * 0.5 / 0.001 = 1_000_000
        let re = reynolds_number(&props, u, l);
        assert!((re - 1_000_000.0).abs() < 1e-9);

        // Pipe Flow (default)
        let pipe_classifier = PipeFlowClassifier;
        assert_eq!(pipe_classifier.classify(re), FlowRegime::Turbulent);

        let re_laminar = reynolds_number(&props, 0.001, 0.1); // Re = 100
        assert_eq!(pipe_classifier.classify(re_laminar), FlowRegime::Laminar);

        // Flat Plate
        let plate_classifier = FlatPlateClassifier;
        // Re = 1,000,000 > 500,000 -> Turbulent
        assert_eq!(plate_classifier.classify(re), FlowRegime::Turbulent);

        // Re = 400,000 -> Laminar for Plate, but Turbulent for Pipe
        let re_intermediate = 400_000.0;
        assert_eq!(
            plate_classifier.classify(re_intermediate),
            FlowRegime::Laminar
        );
        assert_eq!(
            pipe_classifier.classify(re_intermediate),
            FlowRegime::Turbulent
        );
    }

    #[test]
    fn test_bernoulli() {
        let props = FluidProperties::new(1000.0, 1.0); // rho=1000
        let g = 9.81;
        let state = FlowState::new(Vector3::new(10.0, 0.0, 0.0), 101325.0);
        let h = 5.0;

        // P + 0.5 rho v^2 + rho g h
        // 101325 + 500 * 100 + 1000 * 9.81 * 5
        // 101325 + 50000 + 49050 = 200375
        let constant = bernoulli_constant(&state, &props, h, g);
        assert!((constant - 200375.0).abs() < 1e-6);
    }

    #[test]
    fn test_shear_stress() {
        let props = FluidProperties::new(1000.0, 0.001); // mu = 0.001
        let grad_u = 500.0; // 1/s

        // tau = mu * du/dy = 0.001 * 500 = 0.5 Pa
        let tau = shear_stress(&props, grad_u);
        assert!((tau - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_navier_stokes_simple_couette() {
        let props = FluidProperties::new(1.0, 1.0); // rho=1, mu=1 -> nu=1
        let state = FlowState::new(Vector3::new(1.0, 0.0, 0.0), 0.0);

        let vel_grad = Matrix3::zeros();
        let p_grad = Vector3::zeros();
        let lap_vel = Vector3::zeros();
        let g = Vector3::zeros();

        let accel =
            navier_stokes_time_derivative(&props, &state, &vel_grad, p_grad, lap_vel, g).unwrap();
        assert_eq!(accel, Vector3::zeros());

        let p_grad_x = Vector3::new(2.0, 0.0, 0.0);
        let accel_p =
            navier_stokes_time_derivative(&props, &state, &vel_grad, p_grad_x, lap_vel, g).unwrap();
        assert!((accel_p.x - (-2.0)).abs() < 1e-9);
    }

    #[test]
    fn test_continuity() {
        assert_eq!(continuity_divergence(0.0), 0.0);
        assert_eq!(continuity_divergence(0.5), 0.5);
    }
}
