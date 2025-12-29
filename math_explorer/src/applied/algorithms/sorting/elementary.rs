use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Bubble Sort Strategy.
pub struct BubbleSort;

impl<T: Ord + Clone> Sorter<T> for BubbleSort {
    fn name(&self) -> &'static str {
        "Bubble Sort"
    }

    fn sort(&self, data: &[T]) -> SortingResult<T> {
        let mut sorted_data = data.to_vec();
        let mut stats = SortingStats::default();
        let n = sorted_data.len();

        if n < 2 {
            return SortingResult { sorted_data, stats };
        }

        for i in 0..n {
            let mut swapped = false;
            // Optimization: the last i elements are already in place
            for j in 0..n - 1 - i {
                stats.comparisons += 1;
                if sorted_data[j] > sorted_data[j + 1] {
                    sorted_data.swap(j, j + 1);
                    stats.swaps += 1;
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }

        SortingResult { sorted_data, stats }
    }
}

/// Insertion Sort Strategy.
pub struct InsertionSort;

impl<T: Ord + Clone> Sorter<T> for InsertionSort {
    fn name(&self) -> &'static str {
        "Insertion Sort"
    }

    fn sort(&self, data: &[T]) -> SortingResult<T> {
        let mut sorted_data = data.to_vec();
        let mut stats = SortingStats::default();
        let n = sorted_data.len();

        for i in 1..n {
            let mut j = i;
            let temp = sorted_data[i].clone();
            stats.assignments += 1; // temp = ...

            while j > 0 {
                stats.comparisons += 1;
                if sorted_data[j - 1] > temp {
                    sorted_data[j] = sorted_data[j - 1].clone(); // Shift
                    stats.assignments += 1;
                    j -= 1;
                } else {
                    break;
                }
            }
            if j != i {
                sorted_data[j] = temp;
                stats.assignments += 1;
            }
        }

        SortingResult { sorted_data, stats }
    }
}

/// Legacy wrapper for Bubble Sort.
///
/// # Deprecated
/// Use `BubbleSort.sort(data)` instead.
#[deprecated(since = "0.2.0", note = "Use the `BubbleSort` struct implementing the `Sorter` trait.")]
pub fn bubble_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    BubbleSort.sort(data)
}

/// Legacy wrapper for Insertion Sort.
///
/// # Deprecated
/// Use `InsertionSort.sort(data)` instead.
#[deprecated(since = "0.2.0", note = "Use the `InsertionSort` struct implementing the `Sorter` trait.")]
pub fn insertion_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    InsertionSort.sort(data)
}
