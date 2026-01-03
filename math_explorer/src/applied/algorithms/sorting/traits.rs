use super::stats::SortingResult;

/// Defines a strategy for sorting elements.
///
/// This trait allows for the interchangeability of sorting algorithms (Strategy Pattern).
/// Implementations can be passed to functions expecting a generic sorter, enabling
/// benchmarking, testing, and runtime selection of algorithms.
pub trait Sorter<T> {
    /// Sorts the given data and returns the result with statistics.
    ///
    /// # Arguments
    ///
    /// * `data` - The slice of data to be sorted.
    ///
    /// # Returns
    ///
    /// A `SortingResult` containing the sorted data and performance statistics.
    fn sort(&self, data: &[T]) -> SortingResult<T>;
}
