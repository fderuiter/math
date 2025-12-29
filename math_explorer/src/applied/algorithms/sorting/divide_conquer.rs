use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Merge Sort Strategy.
pub struct MergeSort;

impl<T: Ord + Clone> Sorter<T> for MergeSort {
    fn name(&self) -> &'static str {
        "Merge Sort"
    }

    fn sort(&self, data: &[T]) -> SortingResult<T> {
        let mut stats = SortingStats::default();
        let sorted_data = merge_sort_recursive(data, &mut stats);
        SortingResult { sorted_data, stats }
    }
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

/// Quick Sort Strategy.
pub struct QuickSort;

impl<T: Ord + Clone> Sorter<T> for QuickSort {
    fn name(&self) -> &'static str {
        "Quick Sort"
    }

    fn sort(&self, data: &[T]) -> SortingResult<T> {
        let mut sorted_data = data.to_vec();
        let mut stats = SortingStats::default();
        if !sorted_data.is_empty() {
            let n = sorted_data.len();
            quick_sort_recursive(&mut sorted_data, 0, n - 1, &mut stats);
        }
        SortingResult { sorted_data, stats }
    }
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
    let pivot_index = high;
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

/// Legacy wrapper for Merge Sort.
///
/// # Deprecated
/// Use `MergeSort.sort(data)` instead.
#[deprecated(since = "0.2.0", note = "Use the `MergeSort` struct implementing the `Sorter` trait.")]
pub fn merge_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    MergeSort.sort(data)
}

/// Legacy wrapper for Quick Sort.
///
/// # Deprecated
/// Use `QuickSort.sort(data)` instead.
#[deprecated(since = "0.2.0", note = "Use the `QuickSort` struct implementing the `Sorter` trait.")]
pub fn quick_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    QuickSort.sort(data)
}
