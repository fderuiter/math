use super::stats::SortingResult;

/// A Strategy interface for sorting algorithms.
///
/// This trait allows algorithms to be interchanged at runtime or compile-time.
///
/// # Type Parameters
/// * `T` - The type of elements being sorted.
pub trait Sorter<T> {
    /// Sorts the input data and returns the result along with performance statistics.
    ///
    /// # Arguments
    /// * `data` - The slice of data to sort.
    ///
    /// # Returns
    /// * `SortingResult<T>` containing the sorted vector and stats.
    fn sort(&self, data: &[T]) -> SortingResult<T>;
}
