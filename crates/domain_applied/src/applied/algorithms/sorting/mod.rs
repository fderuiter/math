//! # Sorting Algorithms
//!
//! A collection of classic and specialized sorting algorithms, instrumented to provide
//! operational statistics (comparisons, swaps, assignments).
//!
//! This module is designed not just to sort data, but to *teach* and *analyze* sorting behavior.
//! All algorithms return a [`SortingResult`] containing the sorted data and a [`SortingStats`]
//! struct detailed performance metrics.
//!
//! ## Algorithm Comparison
//!
//! | Algorithm | Best Time | Avg Time | Worst Time | Space | Stable? | Constraints |
//! |-----------|-----------|----------|------------|-------|---------|-------------|
//! | [`bubble_sort`] | O(n) | O(n^2) | O(n^2) | O(1) | Yes | None |
//! | [`insertion_sort`] | O(n) | O(n^2) | O(n^2) | O(1) | Yes | None |
//! | [`heap_sort`] | O(n log n) | O(n log n) | O(n log n) | O(1) | No | None |
//! | [`merge_sort`] | O(n log n) | O(n log n) | O(n log n) | O(n) | Yes | None |
//! | [`quick_sort`] | O(n log n) | O(n log n) | O(n^2) | O(log n) | No | None |
//! | [`radix_sort`] | O(nk) | O(nk) | O(nk) | O(n+k) | Yes | `u64` only |
//!
//! *Note: k is the number of digits/bits.*
//!
//! ## Example
//!
//! ```rust
//! use domain_applied::applied::algorithms::sorting::{merge_sort, SortingResult};
//!
//! let data = vec![5, 2, 9, 1, 5, 6];
//!
//! // Perform the sort
//! let result: SortingResult<i32> = merge_sort(&data);
//!
//! assert_eq!(result.sorted_data, vec![1, 2, 5, 5, 6, 9]);
//!
//! // Inspect the cost
//! println!("Comparisons: {}", result.stats.comparisons);
//! println!("Assignments: {}", result.stats.assignments);
//! ```

pub mod divide_conquer;
pub mod elementary;
pub mod heap;
pub mod linear;
pub mod stats;
pub mod strategy;
pub mod theory;

pub use divide_conquer::{merge_sort, quick_sort, MergeSorter, QuickSorter};
pub use elementary::{bubble_sort, insertion_sort, BubbleSorter, InsertionSorter};
pub use heap::{heap_sort, HeapSorter};
pub use linear::{radix_sort, RadixSorter};
pub use stats::{SortingResult, SortingStats};
pub use strategy::Sorter;
pub use theory::{information_theoretic_bound, stirling_approximation_ln_factorial};

// [cite:algorithmic_information_rust]
