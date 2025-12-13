use std::fmt::Debug;

/// Tracks the operational cost of a sorting algorithm.
///
/// This structure allows for empirical verification of theoretical bounds.
#[derive(Debug, Clone, Default)]
pub struct SortingStats {
    /// Number of comparisons performed (e.g., `A[i] > A[j]`).
    pub comparisons: u64,
    /// Number of swaps performed.
    pub swaps: u64,
    /// Number of array writes/assignments (for non-swapping sorts like Merge Sort).
    pub assignments: u64,
}

/// Result of a sorting operation.
pub struct SortingResult<T> {
    /// The sorted data.
    pub sorted_data: Vec<T>,
    /// The statistics collected during the sort.
    pub stats: SortingStats,
}

// -----------------------------------------------------------------------------
// 1. The Mathematical Framework
// -----------------------------------------------------------------------------

/// Calculates the Information Theoretic Lower Bound for comparison-based sorting.
///
/// # Mathematics
///
/// Any comparison sort can be modeled as a decision tree where each node represents a comparison.
/// For a list of `n` distinct elements, there are `n!` possible permutations.
/// To uniquely identify the sorted permutation, the algorithm must distinguish between these outcomes.
/// A binary tree of height `h` has at most `2^h` leaves.
///
/// $$ 2^h \ge n! $$
/// $$ h \ge \log_2(n!) $$
///
/// Using Stirling's Approximation ($\ln n! \approx n \ln n - n$), we derive:
///
/// $$ \log_2(n!) \approx n \log_2 n - 1.44 n $$
///
/// > **Consequence**: No comparison-based sorting algorithm can strictly perform better than $O(n \log n)$ in the worst case.
pub fn information_theoretic_bound(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let n_f = n as f64;
    // Log base 2 of n! approx n * log2(n) - n * log2(e)
    // log2(e) approx 1.442695
    n_f * n_f.log2() - n_f * std::f64::consts::LOG2_E
}

/// Computes Stirling's Approximation for $\ln n!$.
///
/// $$ \ln n! \approx n \ln n - n $$
pub fn stirling_approximation_ln_factorial(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let n_f = n as f64;
    n_f * n_f.ln() - n_f
}

// -----------------------------------------------------------------------------
// 2. Elementary Sorting Algorithms
// -----------------------------------------------------------------------------

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
pub fn bubble_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
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
pub fn insertion_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
    let mut sorted_data = data.to_vec();
    let mut stats = SortingStats::default();
    let n = sorted_data.len();

    for i in 1..n {
        let mut j = i;
        // We count assignments for the temp variable logic if we implemented it that way,
        // but here we use swaps to 'bubble' the element down, which is a common implementation variant.
        // A strict insertion sort shifts elements. Let's implement shift for strict adherence to "Insertion".

        // However, in Rust `swap` is idiomatic. If we want to strictly count "shifts" as assignments:
        // Let's stick to the swap-based implementation often taught, or the shift-based one?
        // Shift based is standard.
        // temp = A[i]
        // while j > 0 and A[j-1] > temp: A[j] = A[j-1]; j--;
        // A[j] = temp;

        // This requires T to be Copy or we clone. We have Clone.

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

// -----------------------------------------------------------------------------
// 3. Efficient Divide and Conquer Algorithms
// -----------------------------------------------------------------------------

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
pub fn heap_sort<T: Ord + Clone>(data: &[T]) -> SortingResult<T> {
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

// -----------------------------------------------------------------------------
// 4. Non-Comparison Sorting (Linear Sorts)
// -----------------------------------------------------------------------------

/// Radix Sort (LSD)
///
/// Sorts integers by processing individual digits.
///
/// # Mathematical Analysis
///
/// * $n$ = number of elements.
/// * $b$ = base (10).
/// * $d$ = number of digits (max value width).
///
/// Time: $d \times O(n + b)$.
/// If $d$ is constant and $b$ is small, this is linear time $O(n)$.
///
/// # constraints
/// Only works for non-negative integers (`u64`) in this implementation.
pub fn radix_sort(data: &[u64]) -> SortingResult<u64> {
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
    for i in 0..n {
        arr[i] = output[i];
        stats.assignments += 1;
    }
}
