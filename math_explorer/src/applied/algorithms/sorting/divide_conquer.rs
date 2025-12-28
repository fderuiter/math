use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Strategy implementation for Merge Sort.
pub struct MergeSort;

impl<T: Ord + Clone> Sorter<T> for MergeSort {
    fn sort(&self, data: &mut [T]) -> SortingStats {
        let mut stats = SortingStats::default();
        let sorted = merge_sort_recursive(data, &mut stats);
        // Copy back to data. This is an overhead of the Strategy pattern for out-of-place algos wrapping as in-place.
        data.clone_from_slice(&sorted);
        stats.assignments += data.len() as u64;
        stats
    }
}

/// Merge Sort
///
/// Divides the array into two halves, recursively sorts them, and merges the sorted halves.
///
/// # Mathematical Analysis
///
/// **Recurrence Relation**: $T(n) = 2T(n/2) + \Theta(n)$.
///
/// Using the **Master Theorem** for $T(n) = aT(n/b) + f(n)$:
/// * $a = 2$ (subproblems)
/// * $b = 2$ (factor of reduction)
/// * $f(n) = n$ (merge cost)
/// * $\log_b a = \log_2 2 = 1$.
/// * Since $f(n) = \Theta(n^{\log_b a})$, we are in Case 2.
/// * Result: $T(n) = \Theta(n \log n)$.
///
/// **Space Complexity**: $O(n)$ auxiliary space.
#[deprecated(since = "0.2.0", note = "Use `MergeSort` struct with `Sorter` trait instead.")]
pub fn merge_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut stats = SortingStats::default();
    let sorted_data = merge_sort_recursive(data, &mut stats);
    SortingResult { sorted_data, stats }
}

fn merge_sort_recursive<T: Ord + Clone>(data: &[T], stats: &mut SortingStats) -> Vec<T> {
    let n = data.len();
    if n <= 1 {
        return data.to_vec();
    }

    let mid = n / 2;
    let left = merge_sort_recursive(&data[0..mid], stats);
    let right = merge_sort_recursive(&data[mid..n], stats);

    merge(&left, &right, stats)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], stats: &mut SortingStats) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        stats.comparisons += 1;
        if left[i] <= right[j] {
            result.push(left[i].clone());
            stats.assignments += 1; // Logic push
            i += 1;
        } else {
            result.push(right[j].clone());
            stats.assignments += 1;
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i].clone());
        stats.assignments += 1;
        i += 1;
    }
    while j < right.len() {
        result.push(right[j].clone());
        stats.assignments += 1;
        j += 1;
    }

    result
}

/// Strategy implementation for Quick Sort.
pub struct QuickSort;

impl<T: Ord + Clone> Sorter<T> for QuickSort {
    fn sort(&self, data: &mut [T]) -> SortingStats {
        let mut stats = SortingStats::default();
        if !data.is_empty() {
            let n = data.len();
            quick_sort_recursive(data, 0, n - 1, &mut stats);
        }
        stats
    }
}

/// Quick Sort
///
/// Selects a pivot and partitions the array.
///
/// # Probabilistic Analysis
///
/// * **Best Case**: Pivot splits array in half. $O(n \log n)$.
/// * **Worst Case**: Pivot is min or max (unbalanced). Recurrence $T(n) = T(n-1) + \Theta(n) \implies O(n^2)$.
/// * **Average Case**: Expected depth is $O(\log n)$.
///
/// **Space Complexity**: $O(\log n)$ stack space (average).
#[deprecated(since = "0.2.0", note = "Use `QuickSort` struct with `Sorter` trait instead.")]
pub fn quick_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut sorted_data = data.to_vec();
    let mut stats = SortingStats::default();
    if !sorted_data.is_empty() {
        let n = sorted_data.len();
        quick_sort_recursive(&mut sorted_data, 0, n - 1, &mut stats);
    }
    SortingResult { sorted_data, stats }
}

fn quick_sort_recursive<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize, stats: &mut SortingStats) {
    if low < high {
        let p = partition(arr, low, high, stats);
        if p > 0 {
            quick_sort_recursive(arr, low, p - 1, stats);
        }
        quick_sort_recursive(arr, p + 1, high, stats);
    }
}

fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize, stats: &mut SortingStats) -> usize {
    // We choose the last element as pivot (Lomuto partition scheme)
    // To avoid worst-case on sorted arrays, random pivot or median-of-three is better,
    // but the prompt describes standard partitioning logic.
    // For "Robustness", let's stick to standard Lomuto but acknowledge pivot choice matters.

    let pivot_index = high;
    // We can't easily move out of slice without unsafe or cloning, so we index.

    let mut i = low;
    for j in low..high {
        stats.comparisons += 1;
        if arr[j] < arr[pivot_index] {
            if i != j {
                arr.swap(i, j);
                stats.swaps += 1;
            }
            i += 1;
        }
    }
    arr.swap(i, high);
    stats.swaps += 1;
    i
}
