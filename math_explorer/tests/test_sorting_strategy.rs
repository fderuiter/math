use math_explorer::applied::algorithms::sorting::*;

#[test]
fn test_strategy_pattern_usage() {
    let data = vec![5, 1, 4, 2, 8];

    // We can now use different strategies interchangeably if we use a generic function
    // or trait objects (though generics are preferred in Rust).

    fn sort_and_verify<S: Sorter<i32>>(sorter: S, data: &[i32]) {
        let result = sorter.sort(data);
        assert_eq!(result.sorted_data, vec![1, 2, 4, 5, 8]);
    }

    sort_and_verify(BubbleSort, &data);
    sort_and_verify(InsertionSort, &data);
    sort_and_verify(MergeSort, &data);
    sort_and_verify(QuickSort, &data);
    sort_and_verify(HeapSort, &data);
}

#[test]
fn test_radix_sort_strategy() {
    let data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let sorter = RadixSort;
    let result = sorter.sort(&data);
    assert_eq!(result.sorted_data, vec![2, 24, 45, 66, 75, 90, 170, 802]);
}

#[test]
#[allow(deprecated)]
fn test_legacy_functions_still_work() {
    let data = vec![5, 1, 4, 2, 8];
    assert_eq!(bubble_sort(&data).sorted_data, vec![1, 2, 4, 5, 8]);
    assert_eq!(quick_sort(&data).sorted_data, vec![1, 2, 4, 5, 8]);
}
