//! # The List Monad
//!
//! In mathematics, the List Monad is often related to the "Free Monoid" or the Power Set functor.
//! In computer science, it represents non-determinism (a computation that can return multiple results).
//!
//! ## Components
//!
//! 1.  **The Functor ($T$):**
//!     For any set $X$, $T(X)$ is the set of all ordered lists of elements from $X$.
//!
//! 2.  **The Unit ($\eta$): "Wrap"**
//!     Takes a single value and wraps it in a list.
//!     $$x \mapsto [x]$$
//!
//! 3.  **The Multiplication ($\mu$): "Flatten"**
//!     Takes a list of lists and concatenates them into a single list.
//!     $$[[x, y], [z]] \mapsto [x, y, z]$$

/// The List Functor $T(X)$.
/// Represents an ordered list of elements of type `T`.
pub type List<T> = Vec<T>;

/// **The Unit ($\eta$): "Wrap"**
///
/// Takes a single value and wraps it in a list.
/// $$x \mapsto [x]$$
pub fn unit<T>(x: T) -> List<T> {
    vec![x]
}

/// **The Multiplication ($\mu$): "Flatten"**
///
/// Takes a list of lists and concatenates them into a single list.
/// $$[[x, y], [z]] \mapsto [x, y, z]$$
pub fn multiplication<T: Clone>(nested_list: List<List<T>>) -> List<T> {
    nested_list.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_unit_law() {
        // Law: Wrapping a value and then flattening should yield the original list?
        // Wait, the user text says:
        // "Let's verify the **Left/Right Unit Law**: *Wrapping a value and then flattening should yield the original list.*"
        // And gives example: Start with L = [1, 2].
        // 1. Apply Unit (eta) to every element: [1, 2] -> [[1], [2]]
        // 2. Apply Multiplication (mu): Flatten -> [1, 2]

        let l = vec![1, 2];

        // 1. Apply Unit (map unit)
        let mapped: List<List<i32>> = l.iter().map(|&x| unit(x)).collect();

        // 2. Apply Multiplication
        let result = multiplication(mapped);

        assert_eq!(result, l);
    }

    #[test]
    fn test_associativity_law() {
        // Law: If we have a triply nested list, it doesn't matter if we flatten inner or outer layers first.
        // Start with L^3 = [[[1, 2]], [[3]]]

        let l3: List<List<List<i32>>> = vec![
            vec![vec![1, 2]],
            vec![vec![3]]
        ];

        // Path 1 (Inner flatten first)
        // Flatten the inside lists [1, 2] and [3] (and the empty ones implied?)
        // The text says: [[[1, 2]], [[3]]] -> [[1, 2], [3]] -> [1, 2, 3]

        // Strictly speaking, "Inner flatten" means applying mu to the inner lists.
        // Map mu over the outer list.
        let inner_flattened: List<List<i32>> = l3.clone().into_iter()
            .map(|inner| multiplication(inner))
            .collect();
        let path1_result = multiplication(inner_flattened);

        // Path 2 (Outer flatten first)
        // Flatten the main container.
        let outer_flattened: List<List<i32>> = multiplication(l3);
        let path2_result = multiplication(outer_flattened);

        assert_eq!(path1_result, vec![1, 2, 3]);
        assert_eq!(path2_result, vec![1, 2, 3]);
        assert_eq!(path1_result, path2_result);
    }
}
