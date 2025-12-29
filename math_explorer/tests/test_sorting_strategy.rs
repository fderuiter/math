
use math_explorer::applied::algorithms::sorting::{
    BubbleSort, HeapSort, InsertionSort, MergeSort, QuickSort, RadixSort, Sorter,
};

#[test]
fn test_all_sorters() {
    let data = vec![5, 1, 4, 2, 8];
    let expected = vec![1, 2, 4, 5, 8];

    // Integer sorters
    let sorters: Vec<Box<dyn Sorter<i32>>> = vec![
        Box::new(BubbleSort),
        Box::new(InsertionSort),
        Box::new(MergeSort),
        Box::new(QuickSort),
        Box::new(HeapSort),
    ];

    for sorter in sorters {
        let result = sorter.sort(&data);
        assert_eq!(result.sorted_data, expected, "Failed for {}", sorter.name());
        // Verify stats are non-zero (simple check)
        if sorter.name() != "Bubble Sort" && sorter.name() != "Insertion Sort" {
            // Comparisons might be 0 for empty array, but here we have data.
            assert!(result.stats.comparisons > 0, "No comparisons for {}", sorter.name());
        }
    }
}

#[test]
fn test_radix_sort() {
    let data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let expected = vec![2, 24, 45, 66, 75, 90, 170, 802];

    let result = RadixSort.sort(&data);
    assert_eq!(result.sorted_data, expected);
}

#[test]
fn test_empty() {
    let data: Vec<i32> = vec![];
    let result = QuickSort.sort(&data);
    assert!(result.sorted_data.is_empty());
}

#[test]
fn test_already_sorted() {
    let data = vec![1, 2, 3, 4, 5];
    let result = BubbleSort.sort(&data);
    assert_eq!(result.sorted_data, data);
    assert!(result.stats.swaps == 0);
}

#[test]
fn test_legacy_functions() {
    let data = vec![3, 1, 2];
    let expected = vec![1, 2, 3];

    #[allow(deprecated)]
    let res = math_explorer::applied::algorithms::sorting::bubble_sort(&data);
    assert_eq!(res.sorted_data, expected);
}
