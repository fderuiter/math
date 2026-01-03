use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Heap Sort Strategy
///
/// # Mathematical Analysis
/// * Time: $\Theta(n \log n)$.
/// * Space: $O(1)$.
pub struct HeapSort;

impl<T: Ord + Clone> Sorter<T> for HeapSort {
    fn sort(&self, data: &[T]) -> SortingResult<T> {
        let mut sorted_data = data.to_vec();
        let mut stats = SortingStats::default();
        let n = sorted_data.len();

        for i in (0..n / 2).rev() {
            heapify(&mut sorted_data, n, i, &mut stats);
        }

        for i in (1..n).rev() {
            sorted_data.swap(0, i);
            stats.swaps += 1;
            heapify(&mut sorted_data, i, 0, &mut stats);
        }

        SortingResult { sorted_data, stats }
    }
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

/// Legacy wrapper for Heap Sort.
#[deprecated(note = "Use HeapSort strategy instead")]
pub fn heap_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    HeapSort.sort(data)
}
