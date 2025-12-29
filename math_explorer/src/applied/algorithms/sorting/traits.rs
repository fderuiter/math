use super::stats::SortingResult;

/// A Strategy trait for sorting algorithms.
///
/// This trait allows for the dynamic selection and interchangeability of sorting algorithms
/// within the library. It follows the Strategy Pattern.
pub trait Sorter<T: Ord + Clone> {
    /// Sorts the provided data and returns the result along with statistics.
    fn sort(&self, data: &[T]) -> SortingResult<T>;

    /// Returns the name of the sorting algorithm.
    fn name(&self) -> &'static str;
}
