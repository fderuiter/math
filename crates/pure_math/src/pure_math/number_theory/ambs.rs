use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmbsError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Modulo error")]
    ModuloError,
}

pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut count = 0;
    while n.is_multiple_of(2) { count += 1; n /= 2; }
    if count > 0 { factors.push((2, count)); }
    let mut i = 3;
    while i * i <= n {
        count = 0;
        while n.is_multiple_of(i) { count += 1; n /= i; }
        if count > 0 { factors.push((i, count)); }
        i += 2;
    }
    if n > 2 { factors.push((n, 1)); }
    factors
}

pub fn solve_quasiperfect_modularity(
    _s_l_str: &str,
    _n_l_str: &str,
    _n_max_str: &str,
    _c_max_int_str: &str,
) -> Result<String, AmbsError> {
    Ok("Not supported on WASM due to GMP limitation".to_string())
}
