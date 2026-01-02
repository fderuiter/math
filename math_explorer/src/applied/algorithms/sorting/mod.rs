pub mod stats;
pub mod theory;
pub mod traits;
pub mod elementary;
pub mod divide_conquer;
pub mod heap;
pub mod linear;

pub use stats::{SortingResult, SortingStats};
pub use traits::Sorter;
pub use theory::{information_theoretic_bound, stirling_approximation_ln_factorial};
pub use elementary::{BubbleSorter, InsertionSorter};
pub use divide_conquer::{MergeSorter, QuickSorter};
pub use heap::{HeapSorter};
pub use linear::{RadixSorter};

// Re-export deprecated functions with suppression to avoid warnings in our own build
#[allow(deprecated)]
pub use elementary::{bubble_sort, insertion_sort};
#[allow(deprecated)]
pub use divide_conquer::{merge_sort, quick_sort};
#[allow(deprecated)]
pub use heap::heap_sort;
#[allow(deprecated)]
pub use linear::radix_sort;
