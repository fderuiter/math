use super::stats::{SortingResult, SortingStats};
use super::traits::Sorter;

/// Radix Sort Strategy (LSD).
///
/// Note: This implementation is specialized for `u64`.
pub struct RadixSort;

impl Sorter<u64> for RadixSort {
    fn name(&self) -> &'static str {
        "Radix Sort"
    }

    fn sort(&self, data: &[u64]) -> SortingResult<u64> {
        let mut sorted_data = data.to_vec();
        let mut stats = SortingStats::default();

        if sorted_data.is_empty() {
            return SortingResult { sorted_data, stats };
        }

        let max_val = *sorted_data.iter().max().unwrap();
        // Comparisons to find max? technically yes, but usually considered O(n) overhead not part of sort loop logic strictly.
        // We'll count them for rigorousness.
        stats.comparisons += sorted_data.len() as u64 - 1;

        let mut exp = 1;
        while max_val / exp > 0 {
            counting_sort_for_radix(&mut sorted_data, exp, &mut stats);
            exp *= 10;
        }

        SortingResult { sorted_data, stats }
    }
}

fn counting_sort_for_radix(arr: &mut [u64], exp: u64, stats: &mut SortingStats) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = [0; 10];

    // Store count of occurrences in count[]
    for &x in arr.iter() {
        let idx = ((x / exp) % 10) as usize;
        count[idx] += 1;
        stats.assignments += 1; // Read/Inc
    }

    // Change count[i] so that count[i] now contains actual
    // position of this digit in output[]
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build the output array
    for i in (0..n).rev() {
        let x = arr[i];
        let idx = ((x / exp) % 10) as usize;
        output[count[idx] - 1] = x;
        count[idx] -= 1;
        stats.assignments += 1; // Move to output
    }

    // Copy the output array to arr[], so that arr[] now
    // contains sorted numbers according to current digit
    arr.copy_from_slice(&output);
    stats.assignments += n as u64;
}

/// Legacy wrapper for Radix Sort.
///
/// # Deprecated
/// Use `RadixSort.sort(data)` instead.
#[deprecated(since = "0.2.0", note = "Use the `RadixSort` struct implementing the `Sorter` trait.")]
pub fn radix_sort(data: &[u64]) -> SortingResult<u64> {
    RadixSort.sort(data)
}
