use super::stats::SortingStats;

/// A strategy for sorting elements.
///
/// This trait allows for the dependency injection of sorting algorithms, enabling
/// runtime selection of strategies based on data characteristics or performance requirements.
pub trait Sorter<T> {
    /// Sorts the provided slice in-place.
    ///
    /// Returns `SortingStats` containing metrics like comparisons and swaps performed.
    fn sort(&self, data: &mut [T]) -> SortingStats;
}
