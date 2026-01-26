//! # Combinatorics (Lemma 1)
//!
//! This module provides the implementation of the **Combinatorial Lemma** from Algorithmic Information Theory papers.
//!
//! ## The Problem
//! Given a set of "bad" elements $V$ and a mapping to a larger space $X$, we want to find a pair $(u, v) \in V \times V$
//! such that they are "dissimilar" according to some similarity functions.
//!
//! This is often used in proofs to show that if a set is large enough, it must contain pairs
//! that are mutually random relative to each other.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub type SimilarityFn<U> = Box<dyn Fn(&U, &U) -> bool>;

/// Implements the Combinatorial Lemma (Lemma 1).
///
/// It searches for a pair $(u, v)$ in the set `v_set` such that:
/// 1. Their mapped neighborhoods in $X$ have a large intersection.
/// 2. But they are dissimilar for a large number of elements in that intersection.
///
/// # Arguments
///
/// * `x_set`: The finite set $X$ (the space of features/witnesses).
/// * `v_set`: The finite set $V$ (the candidates).
/// * `n_v`: A map $N(v) \subseteq X$ (the neighborhood of $v$).
/// * `sim`: A family of similarity relations $S_d(u, v)$ indexed by $d \in X$.
/// * `alpha`: A threshold parameter $\alpha \in (0, 1)$.
///
/// # Returns
///
/// A pair `Some((u, v))` if one exists satisfying the condition:
/// $$ |\{d \in N(u) \cap N(v) : \neg S_d(u, v)\}| > \frac{\alpha^2}{2} |X| $$
///
/// # Example
///
/// ```
/// // See tests/algorithmic_information.rs for a full construction
/// ```
pub fn combinatorial_lemma<T, U>(
    x_set: &HashSet<T>,
    v_set: &HashSet<U>,
    n_v: &HashMap<U, HashSet<T>>,
    sim: &HashMap<T, SimilarityFn<U>>,
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
                if let Some(sim_d) = sim.get(&d)
                    && !sim_d(u, v)
                {
                    count += 1;
                }
            }

            if (count as f64) > threshold {
                return Some((u.clone(), v.clone()));
            }
        }
    }

    None
}
