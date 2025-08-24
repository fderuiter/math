pub mod algebra;
pub mod number_theory;

#[cfg(test)]
mod tests {
    use super::algebra;
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
}
