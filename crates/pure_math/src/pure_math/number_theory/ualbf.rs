use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumberTheoryError {
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub struct UalbfSearchResult {
    pub message: String,
    pub valid_components: usize,
    pub pruned_components: usize,
    pub prefix_count: usize,
    pub rejected_by_lattice: usize,
    pub candidates_checked: usize,
}

pub fn ualbf_search(
    _limit_p: u64,
    _max_exponent: u32,
    _stop_threshold_str: &str,
    _target_max_str: &str,
) -> Result<UalbfSearchResult, NumberTheoryError> {
    Ok(UalbfSearchResult {
        message: "Not supported on WASM due to GMP limitation".to_string(),
        valid_components: 0,
        pruned_components: 0,
        prefix_count: 0,
        rejected_by_lattice: 0,
        candidates_checked: 0,
    })
}
