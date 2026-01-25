extern crate math_explorer;

use math_explorer::applied::battery_degradation;

const TOLERANCE: f64 = 0.3;

#[test]
fn test_n70() {
    let n70_60 = battery_degradation::n70(60.0).unwrap();
    assert!(
        (n70_60 - 576.7).abs() < TOLERANCE,
        "Expected ~576.7, got {}",
        n70_60
    );

    let n70_80 = battery_degradation::n70(80.0).unwrap();
    assert!(
        (n70_80 - 400.9).abs() < TOLERANCE,
        "Expected ~400.9, got {}",
        n70_80
    );

    let n70_100 = battery_degradation::n70(100.0).unwrap();
    assert!(
        (n70_100 - 302.4).abs() < TOLERANCE,
        "Expected ~302.4, got {}",
        n70_100
    );
}

#[test]
fn test_cycles_to_capacity_dod_60() {
    let dod = 60.0;
    let to_90 = battery_degradation::cycles_to_capacity(0.90, dod).unwrap();
    assert!(
        (to_90 - 170.4).abs() < TOLERANCE,
        "DoD 60 to 90%: Expected ~170.4, got {}",
        to_90
    );

    let to_85 = battery_degradation::cycles_to_capacity(0.85, dod).unwrap();
    assert!(
        (to_85 - 262.8).abs() < TOLERANCE,
        "DoD 60 to 85%: Expected ~262.8, got {}",
        to_85
    );

    let to_80 = battery_degradation::cycles_to_capacity(0.80, dod).unwrap();
    assert!(
        (to_80 - 360.8).abs() < TOLERANCE,
        "DoD 60 to 80%: Expected ~360.8, got {}",
        to_80
    );
}

#[test]
fn test_cycles_to_capacity_dod_80() {
    let dod = 80.0;
    let to_90 = battery_degradation::cycles_to_capacity(0.90, dod).unwrap();
    assert!(
        (to_90 - 118.4).abs() < TOLERANCE,
        "DoD 80 to 90%: Expected ~118.4, got {}",
        to_90
    );

    let to_85 = battery_degradation::cycles_to_capacity(0.85, dod).unwrap();
    assert!(
        (to_85 - 182.7).abs() < TOLERANCE,
        "DoD 80 to 85%: Expected ~182.7, got {}",
        to_85
    );

    let to_80 = battery_degradation::cycles_to_capacity(0.80, dod).unwrap();
    assert!(
        (to_80 - 250.8).abs() < TOLERANCE,
        "DoD 80 to 80%: Expected ~250.8, got {}",
        to_80
    );
}

#[test]
fn test_cycles_to_capacity_dod_100() {
    let dod = 100.0;
    let to_90 = battery_degradation::cycles_to_capacity(0.90, dod).unwrap();
    assert!(
        (to_90 - 89.3).abs() < TOLERANCE,
        "DoD 100 to 90%: Expected ~89.3, got {}",
        to_90
    );

    let to_85 = battery_degradation::cycles_to_capacity(0.85, dod).unwrap();
    assert!(
        (to_85 - 137.8).abs() < TOLERANCE,
        "DoD 100 to 85%: Expected ~137.8, got {}",
        to_85
    );

    let to_80 = battery_degradation::cycles_to_capacity(0.80, dod).unwrap();
    assert!(
        (to_80 - 189.2).abs() < TOLERANCE,
        "DoD 100 to 80%: Expected ~189.2, got {}",
        to_80
    );
}
