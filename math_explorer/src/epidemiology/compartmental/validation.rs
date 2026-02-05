use crate::epidemiology::error::EpidemiologyError;

/// Validates that the population size $N$ is strictly positive.
pub fn validate_population(n: f64) -> Result<(), EpidemiologyError> {
    if n <= 0.0 {
        return Err(EpidemiologyError::InvalidParameter {
            name: "n (population)".to_string(),
            value: n,
        });
    }
    Ok(())
}

/// Validates that the initial infected population $I_0$ is non-negative and does not exceed $N$.
pub fn validate_initial_infected(i0: f64, n: f64) -> Result<(), EpidemiologyError> {
    if i0 < 0.0 || i0 > n {
        return Err(EpidemiologyError::InvalidParameter {
            name: "i0 (initial infected)".to_string(),
            value: i0,
        });
    }
    Ok(())
}

/// Validates that a rate parameter (like $\beta$ or $\gamma$) is non-negative.
pub fn validate_rate(rate: f64, name: &str) -> Result<(), EpidemiologyError> {
    if rate < 0.0 {
        return Err(EpidemiologyError::InvalidParameter {
            name: name.to_string(),
            value: rate,
        });
    }
    Ok(())
}
