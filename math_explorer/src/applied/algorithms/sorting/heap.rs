use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Heap Sort Strategy
pub struct HeapSort;

impl<T: Ord + Clone> Sorter<T> for HeapSort {
    fn sort(&self, data: &[T]) -> SortingResult<T> {
        heap_sort_impl(data)
    }
}

/// Heap Sort
///
/// Uses a Binary Heap.
///
/// # Mathematical Steps
///
/// 1. **Heapify**: Convert array to Max-Heap. $O(n)$.
/// 2. **Sort**: Extract max `n` times. Each extraction is $O(\log n)$ (sift down).
///    Total: $O(n \log n)$.
///
/// Total Complexity: $O(n) + O(n \log n) = \Theta(n \log n)$.
/// Space: $O(1)$ (in-place).
#[deprecated(since = "0.2.0", note = "Use `HeapSort.sort(data)` strategy instead.")]
pub fn heap_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    heap_sort_impl(data)
}

fn heap_sort_impl<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut sorted_data = data.to_vec();
    let mut stats = SortingStats::default();
    let n = sorted_data.len();

    // Build heap (rearrange array)
    for i in (0..n / 2).rev() {
        heapify(&mut sorted_data, n, i, &mut stats);
    }

    // One by one extract an element from heap
    for i in (1..n).rev() {
        // Move current root to end
        sorted_data.swap(0, i);
        stats.swaps += 1;

        // call max heapify on the reduced heap
        heapify(&mut sorted_data, i, 0, &mut stats);
    }

    SortingResult { sorted_data, stats }
}

fn heapify<T: Ord + Clone>(arr: &mut [T], n: usize, i: usize, stats: &mut SortingStats) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n {
        stats.comparisons += 1;
        if arr[l] > arr[largest] {
            largest = l;
        }
    }

    if r < n {
        stats.comparisons += 1;
        if arr[r] > arr[largest] {
            largest = r;
        }
    }

    if largest != i {
        arr.swap(i, largest);
        stats.swaps += 1;
        heapify(arr, n, largest, stats);
    }
}
