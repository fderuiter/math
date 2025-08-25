pub mod algebra;
pub mod favoritism;
pub mod number_theory;

#[cfg(test)]
mod tests {
    use super::algebra;
    use super::favoritism::{self, FavoritismInputs};
    use super::number_theory;

    #[test]
    fn test_algebra_placeholder() {
        assert_eq!(algebra::placeholder_add(5, 3), 8);
    }

    #[test]
    fn test_number_theory_placeholder() {
        assert!(number_theory::is_prime_placeholder(2));
        assert!(!number_theory::is_prime_placeholder(10));
    }

    #[test]
    fn test_favoritism_score() {
        let inputs = FavoritismInputs::default();
        let score = favoritism::calculate_favoritism_score(&inputs);
        // The python script gives a value around 80,575,561
        // We test for a range to account for the random factor
        assert!(score > 72518005.0 && score < 88633118.0);
    }
}
