use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Strategy implementation for Bubble Sort.
pub struct BubbleSort;

impl<T: Ord + Clone> Sorter<T> for BubbleSort {
    fn sort(&self, data: &mut [T]) -> SortingStats {
        let mut stats = SortingStats::default();
        let n = data.len();

        if n < 2 {
            return stats;
        }

        for i in 0..n {
            let mut swapped = false;
            // Optimization: the last i elements are already in place
            for j in 0..n - 1 - i {
                stats.comparisons += 1;
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                    stats.swaps += 1;
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
        stats
    }
}

/// Bubble Sort
///
/// Repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order.
///
/// # Mathematical Analysis
///
/// * **Comparisons**: $\sum_{i=1}^{n-1} i = \frac{n(n-1)}{2} = \Theta(n^2)$.
/// * **Swaps (Worst Case)**: Reverse order implies every comparison leads to a swap: $\frac{n(n-1)}{2} = \Theta(n^2)$.
/// * **Swaps (Best Case)**: Sorted order implies 0 swaps.
/// * **Stability**: Yes.
#[deprecated(since = "0.2.0", note = "Use `BubbleSort` struct with `Sorter` trait instead.")]
pub fn bubble_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut sorted_data = data.to_vec();
    let stats = BubbleSort.sort(&mut sorted_data);
    SortingResult { sorted_data, stats }
}

/// Strategy implementation for Insertion Sort.
pub struct InsertionSort;

impl<T: Ord + Clone> Sorter<T> for InsertionSort {
    fn sort(&self, data: &mut [T]) -> SortingStats {
        let mut stats = SortingStats::default();
        let n = data.len();

        for i in 1..n {
            let mut j = i;
            let temp = data[i].clone();
            stats.assignments += 1; // temp = ...

            while j > 0 {
                stats.comparisons += 1;
                if data[j - 1] > temp {
                    data[j] = data[j - 1].clone(); // Shift
                    stats.assignments += 1;
                    j -= 1;
                } else {
                    break;
                }
            }
            if j != i {
                data[j] = temp;
                stats.assignments += 1;
            }
        }
        stats
    }
}

/// Insertion Sort
///
/// Builds the final sorted array one item at a time.
///
/// # Mathematical Analysis
///
/// * **Worst Case**: Reverse order. To insert the k-th element, we shift k-1 elements.
///   Total time: $\sum_{k=2}^{n} (k-1) = \Theta(n^2)$.
/// * **Best Case**: Sorted order. 1 comparison per element, 0 swaps. $\Theta(n)$.
/// * **Stability**: Yes.
#[deprecated(since = "0.2.0", note = "Use `InsertionSort` struct with `Sorter` trait instead.")]
pub fn insertion_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut sorted_data = data.to_vec();
    let stats = InsertionSort.sort(&mut sorted_data);
    SortingResult { sorted_data, stats }
}
