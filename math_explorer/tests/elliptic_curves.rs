use math_explorer::pure_math::elliptic_curves::*;
use std::collections::HashMap;

#[test]
fn test_prime_factors() {
    let mut factors = HashMap::new();
    factors.insert(2, 2);
    factors.insert(3, 1);
    factors.insert(5, 1);
    assert_eq!(prime_factors(60), factors);

    let mut factors_2 = HashMap::new();
    factors_2.insert(97, 1);
    assert_eq!(prime_factors(97), factors_2);

    let factors_3 = HashMap::new();
    assert_eq!(prime_factors(1), factors_3);

    let factors_4 = HashMap::new();
    assert_eq!(prime_factors(0), factors_4);
}

#[test]
fn test_psi_function() {
    assert_eq!(psi(1), 1);
    assert_eq!(psi(2), 3);
    assert_eq!(psi(3), 4);
    assert_eq!(psi(4), 6);
    assert_eq!(psi(5), 6);
    assert_eq!(psi(6), 12);
    assert_eq!(psi(10), 18);
    assert_eq!(psi(11), 12);
}

#[test]
fn test_theorem_1_1_bounds_simple() {
    // For N=5, psi(5) = 6. Let's check a_{1,1} where i+j=2. diff = 4
    let bounds = theorem_1_1_bounds(5, 1, 1).unwrap();
    assert_eq!(bounds.v2_bound, Some(15 * 4)); // 60
    assert_eq!(bounds.v3_bound, Some(3 * 4)); // 12
    assert_eq!(bounds.v5_bound, None);
}

#[test]
fn test_theorem_1_1_bounds_n_is_1_mod_3() {
    // For N=7, psi(7) = 8. N=7 is 1 mod 3. Check a_{2,1} where i+j=3. diff = 5
    let bounds = theorem_1_1_bounds(7, 2, 1).unwrap();
    assert_eq!(bounds.v2_bound, Some(15 * 5)); // 75
    // ceil(4.5 * 5) = ceil(22.5) = 23
    assert_eq!(bounds.v3_bound, Some(23));
    assert_eq!(bounds.v5_bound, Some(3 * 5)); // 15
}

#[test]
fn test_theorem_1_1_bounds_panic() {
    // N=5, psi(5)=6. i+j=6, which should error.
    assert!(theorem_1_1_bounds(5, 3, 3).is_err());
}

#[test]
fn test_theorem_1_2_bounds_simple() {
    // For N=11, psi(11) = 12. N=11 is 3 mod 4. Check a_{1,0}. diff = 11
    let bounds = theorem_1_2_bounds(11, 1, 0).unwrap();
    assert_eq!(bounds.v2_bound, Some(9 * 11)); // 99
    assert_eq!(bounds.v3_bound, Some(6 * 11)); // 66
    assert_eq!(bounds.v7_bound, Some(2 * 11)); // 22
}

#[test]
fn test_theorem_1_2_bounds_n_is_1_mod_4() {
    // For N=5, psi(5) = 6. N=5 is 1 mod 4. Check a_{0,0}. diff = 6
    let bounds = theorem_1_2_bounds(5, 0, 0).unwrap();
    assert_eq!(bounds.v2_bound, Some(10 * 6)); // 60
    assert_eq!(bounds.v3_bound, Some(6 * 6)); // 36
    assert_eq!(bounds.v7_bound, Some(2 * 6)); // 12
}

#[test]
fn test_theorem_1_2_bounds_panic() {
    // N=5, psi(5)=6. i+j=7, which should error.
    assert!(theorem_1_2_bounds(5, 4, 3).is_err());
}
