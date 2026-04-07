//! # Unified Algebraic-Lattice Bipartition Framework (UALBF)
//!
//! A synthesis of the Algebraic-Lattice Convergence Framework (ALCF) and the
//! Algebraic-Modular Bipartition Sieve (AMBS) for proving lower bounds on
//! quasiperfect numbers.
//!
//! ## Overview
//!
//! A quasiperfect number N satisfies σ(N) = 2N + 1. This framework searches
//! for such numbers by:
//!
//! 1. **Phase 1** – Global Annihilation Sieve: filter prime-power components
//! 2. **Phase 2** – Prefix Tree: build factorization prefixes multiplicatively
//! 3. **Phase 3** – Lattice Oracle: reject prefixes failing modular existence
//! 4. **Phase 4** – Exact Ray Casting: search for exact quasiperfect candidates
//!
//! ## References
//!
//! - Guy, R. K. (2004). *Unsolved Problems in Number Theory* (3rd ed.), Problem B2.
//! - Cohen, G. L., & Hagis, P. (1982). Results concerning odd perfect numbers.
use crate::pure_math::number_theory::alcf::sigma;
use crate::pure_math::number_theory::ambs::{AmbsDsp, modular_inverse, prime_factors};
use crate::pure_math::number_theory::primes::is_prime;
use rug::{Integer, ops::RemRounding};

/// A single prime-power component p^(2e) that survived the global annihilation sieve.
#[allow(dead_code)]
pub struct PrimePower {
    pub p: u64,
    pub exponent: u32,
    pub val: u64,
    pub sigma_val: u64,
}

/// A prefix of a quasiperfect factorization built from a subset of `PrimePower` components.
#[allow(dead_code)]
pub struct Prefix {
    pub n_l: Integer,
    pub s_l: Integer,
    pub factors: Vec<u64>,
}

/// Aggregated statistics returned by a full UALBF search run.
pub struct UalbfSearchResult {
    pub valid_components: usize,
    pub pruned_components: usize,
    pub prefix_count: usize,
    pub rejected_by_lattice: usize,
    pub candidates_checked: usize,
}

// ─── Phase 1 ────────────────────────────────────────────────────────────────

/// **Phase 1 — Global Annihilation Sieve**
///
/// Enumerates all odd prime powers p^(2e) with p ≤ `limit_p` and 1 ≤ e ≤ `max_e`,
/// retaining only those whose sum-of-divisors σ(p^(2e)) has *no* prime factor
/// q satisfying q ≡ 5 (mod 8) or q ≡ 7 (mod 8).
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::number_theory::ualbf::phase1_global_annihilation_sieve;
/// let (valid, pruned) = phase1_global_annihilation_sieve(20, 1);
/// assert!(pruned > 0);
/// ```
pub fn phase1_global_annihilation_sieve(limit_p: u64, max_e: u32) -> (Vec<PrimePower>, usize) {
    let mut valid: Vec<PrimePower> = Vec::new();
    let mut pruned: usize = 0;

    let mut p = 3u64;
    while p <= limit_p {
        if is_prime(p) {
            for e in 1u32..=max_e {
                let exp2e = 2u32 * e;
                let val = match (p as u128).checked_pow(exp2e) {
                    Some(v) if v <= u64::MAX as u128 => v as u64,
                    _ => break,
                };

                let sigma_val = sigma(val);
                let factors = prime_factors(sigma_val);
                let annihilated = factors.iter().any(|&(q, _)| q % 8 == 5 || q % 8 == 7);

                if annihilated {
                    pruned += 1;
                } else {
                    valid.push(PrimePower {
                        p,
                        exponent: e,
                        val,
                        sigma_val,
                    });
                }
            }
        }
        p += 2;
    }

    valid.sort_by_key(|c| c.val);
    (valid, pruned)
}

// ─── Phase 2 ────────────────────────────────────────────────────────────────

/// **Phase 2 — Prefix Tree Construction**
///
/// Builds all factorisation prefixes whose product n_l ≥ `stop_threshold`.
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::number_theory::ualbf::{phase1_global_annihilation_sieve, phase2_build_prefix_tree};
/// use rug::Integer;
/// let (comps, _) = phase1_global_annihilation_sieve(20, 1);
/// let prefixes = phase2_build_prefix_tree(&comps, &Integer::from(50u64));
/// for p in &prefixes { assert!(p.n_l >= Integer::from(50u64)); }
/// ```
pub fn phase2_build_prefix_tree(
    components: &[PrimePower],
    stop_threshold: &Integer,
) -> Vec<Prefix> {
    let mut pool: Vec<Prefix> = Vec::new();

    let mut stack: Vec<Prefix> = vec![Prefix {
        n_l: Integer::from(1u64),
        s_l: Integer::from(1u64),
        factors: vec![],
    }];

    while let Some(prefix) = stack.pop() {
        if prefix.n_l >= *stop_threshold {
            pool.push(prefix);
            continue;
        }

        let mut extended = false;
        for comp in components {
            if prefix.factors.contains(&comp.p) {
                continue;
            }

            let new_n_l = Integer::from(&prefix.n_l * comp.val);
            let new_s_l = Integer::from(&prefix.s_l * comp.sigma_val);
            let mut new_factors = prefix.factors.clone();
            new_factors.push(comp.p);

            stack.push(Prefix {
                n_l: new_n_l,
                s_l: new_s_l,
                factors: new_factors,
            });
            extended = true;
        }

        if !extended {
            pool.push(prefix);
        }
    }

    pool
}

// ─── Phase 3 ────────────────────────────────────────────────────────────────

/// **Phase 3 — Lattice Oracle**
///
/// Returns `true` if the prefix is *rejected* (cannot possibly lead to a quasiperfect number).
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::number_theory::ualbf::{Prefix, phase3_lattice_oracle_rejects};
/// use rug::Integer;
/// let prefix = Prefix { n_l: Integer::from(49u64), s_l: Integer::from(57u64), factors: vec![7] };
/// let _ = phase3_lattice_oracle_rejects(&prefix);
/// ```
pub fn phase3_lattice_oracle_rejects(prefix: &Prefix) -> bool {
    if prefix.s_l == 1 {
        return false;
    }

    let neg_two_nl = Integer::from(-2i32) * &prefix.n_l;
    let x_l_inv = modular_inverse(&neg_two_nl, &prefix.s_l);
    if x_l_inv.is_none() {
        return true;
    }

    if prefix.n_l == 1 {
        return false;
    }

    let y_l_inv = modular_inverse(&prefix.s_l, &prefix.n_l);
    if y_l_inv.is_none() {
        return true;
    }

    false
}

// ─── Phase 4 ────────────────────────────────────────────────────────────────

fn phase4_inner(prefix: &Prefix, target_max: &Integer) -> (Option<Integer>, usize) {
    let neg_two_nl = Integer::from(-2i32) * &prefix.n_l;
    let x_l = match modular_inverse(&neg_two_nl, &prefix.s_l) {
        Some(v) => v,
        None => return (None, 0),
    };

    let x_l_reduced = x_l.clone().rem_euc(&prefix.s_l);

    let roots = match AmbsDsp::compute_modular_square_roots(&x_l_reduced, &prefix.s_l) {
        Ok(r) => r,
        Err(_) => return (None, 0),
    };
    if roots.is_empty() {
        return (None, 0);
    }

    let target_max_sqrt = target_max.clone().sqrt();
    let mut candidates_checked: usize = 0;

    for r_i in &roots {
        let mut c: u64 = 0;
        loop {
            let offset = Integer::from(c) * &prefix.s_l;
            let z = Integer::from(r_i + &offset);

            if z > target_max_sqrt {
                break;
            }
            candidates_checked += 1;

            let z_sq = Integer::from(&z * &z);
            let two_nl = Integer::from(2u32) * &prefix.n_l;
            let lhs = two_nl * &z_sq + Integer::from(1u32);

            let r = lhs.clone().rem_euc(&prefix.s_l);
            if r == 0 {
                let candidate = Integer::from(&prefix.n_l * &z_sq);
                return (Some(candidate), candidates_checked);
            }

            c += 1;
        }
    }

    (None, candidates_checked)
}

/// **Phase 4 — Exact Ray Casting**
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::number_theory::ualbf::{Prefix, phase4_exact_ray_casting};
/// use rug::Integer;
/// let prefix = Prefix { n_l: Integer::from(49u64), s_l: Integer::from(57u64), factors: vec![7] };
/// let _ = phase4_exact_ray_casting(&prefix, &Integer::from(1_000_000u64));
/// ```
pub fn phase4_exact_ray_casting(prefix: &Prefix, target_max: &Integer) -> Option<Integer> {
    let (candidate, _) = phase4_inner(prefix, target_max);
    candidate
}

// ─── Full pipeline ───────────────────────────────────────────────────────────

/// **UALBF Search**
///
/// Executes the complete four-phase pipeline and returns aggregated statistics.
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::number_theory::ualbf::ualbf_search;
/// let result = ualbf_search(50, 1, "1000", "1000000");
/// assert!(result.valid_components > 0 || result.pruned_components > 0);
/// ```
use crate::pure_math::number_theory::error::NumberTheoryError;

pub fn ualbf_search(
    limit_p: u64,
    max_e: u32,
    stop_threshold_str: &str,
    target_max_str: &str,
) -> Result<UalbfSearchResult, NumberTheoryError> {
    let stop_threshold: Integer = Integer::parse(stop_threshold_str)
        .map_err(|e| NumberTheoryError::ParseError(e.to_string()))?
        .into();
    let target_max: Integer = Integer::parse(target_max_str)
        .map_err(|e| NumberTheoryError::ParseError(e.to_string()))?
        .into();

    let (components, pruned_components) = phase1_global_annihilation_sieve(limit_p, max_e);
    let valid_components = components.len();

    let prefixes = phase2_build_prefix_tree(&components, &stop_threshold);
    let prefix_count = prefixes.len();

    let mut rejected_by_lattice: usize = 0;
    let mut candidates_checked: usize = 0;

    for prefix in &prefixes {
        if phase3_lattice_oracle_rejects(prefix) {
            rejected_by_lattice += 1;
        } else {
            let (_, checked) = phase4_inner(prefix, &target_max);
            candidates_checked += checked;
        }
    }

    Ok(UalbfSearchResult {
        valid_components,
        pruned_components,
        prefix_count,
        rejected_by_lattice,
        candidates_checked,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Integer;

    #[test]
    fn test_phase1_sieve_prunes_known() {
        let (valid, pruned) = phase1_global_annihilation_sieve(20, 1);
        assert!(pruned > 0, "expected pruned components");
        let valid_ps: Vec<u64> = valid.iter().map(|c| c.p).collect();
        assert!(!valid_ps.contains(&3), "p=3 should be pruned");
        assert!(!valid_ps.contains(&5), "p=5 should be pruned");
        assert!(valid_ps.contains(&7), "p=7 should survive");
    }

    #[test]
    fn test_phase2_prefix_tree_small() {
        let (components, _) = phase1_global_annihilation_sieve(20, 1);
        let threshold = Integer::from(50u64);
        let prefixes = phase2_build_prefix_tree(&components, &threshold);
        assert!(!prefixes.is_empty(), "expected at least one prefix");
        for p in &prefixes {
            assert!(
                p.n_l >= threshold,
                "prefix n_l={} below threshold={}",
                p.n_l,
                threshold
            );
        }
    }

    #[test]
    fn test_phase3_oracle_rejects_trivial() {
        let prefix = Prefix {
            n_l: Integer::from(1u64),
            s_l: Integer::from(1u64),
            factors: vec![],
        };
        let result = phase3_lattice_oracle_rejects(&prefix);
        assert!(!result, "degenerate prefix should not be rejected");
    }

    #[test]
    fn test_phase3_oracle_real_prefix() {
        let prefix = Prefix {
            n_l: Integer::from(49u64),
            s_l: Integer::from(57u64),
            factors: vec![7],
        };
        let _ = phase3_lattice_oracle_rejects(&prefix);
    }

    #[test]
    fn test_ualbf_search_runs() {
        let result = ualbf_search(50, 1, "1000", "1000000").unwrap();
        assert!(
            result.valid_components > 0 || result.pruned_components > 0,
            "pipeline produced no output"
        );
    }
}
