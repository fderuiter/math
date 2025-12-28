use super::stats::SortingResult;

/// Defines a strategy for sorting elements.
///
/// This trait allows for the interchangeability of sorting algorithms
/// in the `math_explorer` system.
pub trait Sorter<T> {
    /// Sorts the data and returns the result with statistics.
    ///
    /// # Arguments
    /// * `data` - The slice of data to sort.
    ///
    /// # Returns
    /// * `SortingResult<T>` containing the sorted data and operational stats.
    fn sort(&self, data: &[T]) -> SortingResult<T>;
}
