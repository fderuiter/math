#[allow(deprecated)]
use domain_applied::applied::game_theory::equilibrium::{
    BestResponseCorrespondence, BoxSet, ConvexSet, FixedPointVerifier,
};
use domain_applied::applied::game_theory::evolutionary::ReplicatorDynamics;
use domain_applied::applied::game_theory::mean_field::{Density, MeanFieldGame1D, Position};
#[allow(deprecated)]
use domain_applied::applied::game_theory::mechanism_design::MechanismDesign;
use nalgebra::{DMatrix, DVector};
use statrs::distribution::Uniform;

#[test]
#[allow(deprecated)]
fn test_equilibrium_integration() {
    let box_set = BoxSet::new(vec![0.0], vec![1.0]);
    assert!(box_set.contains(&DVector::from_vec(vec![0.5])));

    // Check trivial fixed point
    let correspondence = BestResponseCorrespondence {
        mapping: Box::new(|x| x.clone()),
        tolerance: 1e-6,
    };
    assert!(FixedPointVerifier::is_fixed_point(
        &correspondence,
        &DVector::from_vec(vec![0.5])
    ));
}

#[test]
fn test_mean_field_integration() {
    let mfg = MeanFieldGame1D::new(0.1, 1.0, 10, 10, -1.0, 1.0);
    // Just ensure it runs without panic
    let _res = mfg.solve(
        |_p: Position, d: Density| d.0 * d.0,
        |_p: Position, _d: Density| 0.0,
        |_p: Position| 1.0,
        2,
    );
}

#[test]
fn test_evolutionary_integration() {
    let payoff = DMatrix::from_row_slice(2, 2, &[3.0, 0.0, 5.0, 1.0]); // Prisoner's Dilemma-ish
    let system = ReplicatorDynamics::new(payoff).unwrap();
    let traj = system.simulate(DVector::from_vec(vec![0.5, 0.5]), 1.0, 0.1);
    assert!(!traj.is_empty());
}

#[test]
#[allow(deprecated)]
fn test_mechanism_integration() {
    let dist = Uniform::new(0.0, 1.0).unwrap();
    let r = MechanismDesign::optimal_reserve_price(&dist, 0.0, 1.0);
    assert!((r - 0.5).abs() < 1e-2);
}
