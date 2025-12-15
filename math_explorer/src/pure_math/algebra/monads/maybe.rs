//! # The Maybe Monad
//!
//! This structure represents computations that might fail.
//!
//! ## Components
//!
//! 1.  **The Functor ($T$):**
//!     For any set $X$, $T(X) = X \cup \{\text{Nothing}\}$.
//!     (Every element is either a value "Just $x$" or the null value "Nothing").
//!
//! 2.  **The Unit ($\eta$):**
//!     Takes a value and wraps it in the "Just" context.
//!     $$x \mapsto \text{Just } x$$
//!
//! 3.  **The Multiplication ($\mu$):**
//!     Collapses two layers of "Maybe".
//!     * $\text{Just }(\text{Just } x) \mapsto \text{Just } x$
//!     * $\text{Just }(\text{Nothing}) \mapsto \text{Nothing}$
//!     * $\text{Nothing} \mapsto \text{Nothing}$

/// The Maybe Functor $T(X)$.
/// Represents a value that may or may not exist.
pub type Maybe<T> = Option<T>;

/// **The Unit ($\eta$):**
///
/// Takes a value and wraps it in the "Just" context.
/// $$x \mapsto \text{Just } x$$
pub fn unit<T>(x: T) -> Maybe<T> {
    Some(x)
}

/// **The Multiplication ($\mu$):**
///
/// Collapses two layers of "Maybe".
pub fn multiplication<T>(nested_maybe: Maybe<Maybe<T>>) -> Maybe<T> {
    match nested_maybe {
        Some(inner) => inner,
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_unit_law() {
        // Law: Wrapping a value and then flattening returns the value.
        // 1. Start with value x = 5.
        // 2. Wrap (eta): Just 5
        // 3. Wrap again (conceptually): Just (Just 5)
        // 4. Flatten (mu): Just 5

        let x = 5;
        let wrapped = unit(x);
        let double_wrapped = unit(wrapped); // Just (Just 5)

        let result = multiplication(double_wrapped);

        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_associativity_law_failure() {
        // Law: Associativity with a failure case.
        // M = Just (Just (Nothing))

        // Construct M: Just (Just (Nothing))
        // Inner Nothing is Maybe<i32> (None)
        // Middle is Just(None) -> Some(None)
        // Outer is Just(Some(None)) -> Some(Some(None))

        let nothing: Maybe<i32> = None;
        let m: Maybe<Maybe<Maybe<i32>>> = Some(Some(nothing));

        // Path 1 (Flatten outer first)
        // The outer Just meets the middle Just. They collapse.
        // Just (Just (Nothing)) -> Just (Nothing)
        let outer_flattened = multiplication(m.clone()); // Should be Some(None)

        // Flatten again: Just (Nothing) -> Nothing
        let path1_result = multiplication(outer_flattened);

        // Path 2 (Flatten inner first)
        // Look strictly at the inner term: Just (Nothing). This collapses to Nothing.
        // Now we have the outer layer wrapping that result:
        // Just (... result ...) -> Just (Nothing)

        // To implement "flatten inner first", we need to map multiplication over the outer layer.
        // Since `Maybe` is an Option, we can use `map`.
        let inner_flattened: Maybe<Maybe<i32>> = m.map(|inner| multiplication(inner));

        // Flatten again: Just (Nothing) -> Nothing
        let path2_result = multiplication(inner_flattened);

        assert_eq!(path1_result, None);
        assert_eq!(path2_result, None);
    }
}
