#[allow(deprecated)]
use domain_physics::physics::astrophysics::galaxies::{
    calculate_apparent_magnitude_from_distance, calculate_log_mass_from_absolute_magnitude,
    calculate_log_mass_from_distance, calculate_redshift_from_log_mass, GeneralIrregular,
    TypeCode10, TypeCode95To99,
};

const F64_TOLERANCE: f64 = 1e-9;

#[test]
#[allow(deprecated)]
fn test_calculate_log_mass_from_distance() {
    // Test case for GeneralIrregular
    let distance = 10.0; // Mpc
    let expected = 0.0230 * distance + 0.7840;
    let result = calculate_log_mass_from_distance(distance, &GeneralIrregular);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode10
    let expected = 0.0250 * distance + 7.6860;
    let result = calculate_log_mass_from_distance(distance, &TypeCode10);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode95To99
    let expected = 0.0504 * distance + 7.5715;
    let result = calculate_log_mass_from_distance(distance, &TypeCode95To99);
    assert!((result - expected).abs() < F64_TOLERANCE);
}

#[test]
#[allow(deprecated)]
fn test_calculate_apparent_magnitude_from_distance() {
    let distance = 10.0; // Mpc

    // Test case for GeneralIrregular
    let expected = 0.0206 * distance + 16.0010;
    let result = calculate_apparent_magnitude_from_distance(distance, &GeneralIrregular);
    assert!((result.unwrap() - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode10
    let expected = 0.0140 * distance + 16.575;
    let result = calculate_apparent_magnitude_from_distance(distance, &TypeCode10);
    assert!((result.unwrap() - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode95To99 (should be None)
    let result = calculate_apparent_magnitude_from_distance(distance, &TypeCode95To99);
    assert!(result.is_none());
}

#[test]
#[allow(deprecated)]
fn test_calculate_log_mass_from_absolute_magnitude() {
    let absolute_magnitude_v = -15.0;

    // Test case for GeneralIrregular
    let expected = -0.6670 * absolute_magnitude_v - 1.4975;
    let result =
        calculate_log_mass_from_absolute_magnitude(absolute_magnitude_v, &GeneralIrregular);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode10
    let result = calculate_log_mass_from_absolute_magnitude(absolute_magnitude_v, &TypeCode10);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode95To99
    let expected = -0.3837 * absolute_magnitude_v - 2.2864;
    let result = calculate_log_mass_from_absolute_magnitude(absolute_magnitude_v, &TypeCode95To99);
    assert!((result - expected).abs() < F64_TOLERANCE);
}

#[test]
#[allow(deprecated)]
fn test_calculate_redshift_from_log_mass() {
    let log_mass_solar = 10.0;

    // Test case for GeneralIrregular
    let expected = 0.0094 * log_mass_solar - 0.7270;
    let result = calculate_redshift_from_log_mass(log_mass_solar, &GeneralIrregular);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode10
    let expected = 0.00093 * log_mass_solar - 0.0716;
    let result = calculate_redshift_from_log_mass(log_mass_solar, &TypeCode10);
    assert!((result - expected).abs() < F64_TOLERANCE);

    // Test case for TypeCode95To99
    let expected = 0.0031 * log_mass_solar - 0.0223;
    let result = calculate_redshift_from_log_mass(log_mass_solar, &TypeCode95To99);
    assert!((result - expected).abs() < F64_TOLERANCE);
}
