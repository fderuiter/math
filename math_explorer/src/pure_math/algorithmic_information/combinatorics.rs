//! # Combinatorics
//!
//! This module provides the implementation of the Combinatorial Lemma (Lemma 1) from the paper.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Implements the Combinatorial Lemma (Lemma 1).
///
/// # Arguments
///
/// * `x_set`: A finite set X.
/// * `v_set`: A finite set V.
/// * `n_v`: A map from each v in V to a subset of X.
/// * `sim`: A map from each d in X to a similarity relation on V.
/// * `alpha`: A parameter in (0, 1).
///
/// # Returns
///
/// A tuple (u, v) from V that satisfies the lemma's conclusion, or None if no such pair exists.
pub fn combinatorial_lemma<T, U>(
    x_set: &HashSet<T>,
    v_set: &HashSet<U>,
    n_v: &HashMap<U, HashSet<T>>,
    sim: &HashMap<T, Box<dyn Fn(&U, &U) -> bool>>,
    alpha: f64,
) -> Option<(U, U)>
where
    T: Eq + Hash + Clone,
    U: Eq + Hash + Clone,
{
    let x_size = x_set.len() as f64;
    let threshold = (alpha * alpha / 2.0) * x_size;

    for u in v_set {
        for v in v_set {
            let n_u = match n_v.get(u) {
                Some(s) => s,
                None => continue,
            };
            let n_v = match n_v.get(v) {
                Some(s) => s,
                None => continue,
            };

            let intersection: HashSet<_> = n_u.intersection(n_v).cloned().collect();

            let mut count = 0;
            for d in intersection {
                if let Some(sim_d) = sim.get(&d) {
                    if !sim_d(u, v) {
                        count += 1;
                    }
                }
            }

            if (count as f64) > threshold {
                return Some((u.clone(), v.clone()));
            }
        }
    }

    None
}
