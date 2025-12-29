pub mod stats;
pub mod theory;
pub mod elementary;
pub mod divide_conquer;
pub mod heap;
pub mod linear;
pub mod traits;

pub use stats::{SortingResult, SortingStats};
pub use theory::{information_theoretic_bound, stirling_approximation_ln_factorial};
#[allow(deprecated)]
pub use elementary::{bubble_sort, insertion_sort, BubbleSort, InsertionSort};
#[allow(deprecated)]
pub use divide_conquer::{merge_sort, quick_sort, MergeSort, QuickSort};
#[allow(deprecated)]
pub use heap::{heap_sort, HeapSort};
#[allow(deprecated)]
pub use linear::{radix_sort, RadixSort};
pub use traits::Sorter;
