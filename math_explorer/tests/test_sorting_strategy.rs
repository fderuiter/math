#[cfg(test)]
mod tests {
    use math_explorer::applied::algorithms::sorting::{
        Sorter, BubbleSorter, QuickSorter, RadixSorter, SortingResult
    };

    // Generic function demonstrating the Strategy Pattern
    // It accepts ANY Sorter capable of sorting T.
    fn run_strategy<T, S>(sorter: S, data: &[T]) -> SortingResult<T>
    where
        S: Sorter<T>,
        T: PartialEq + std::fmt::Debug, // For assertion
    {
        sorter.sort(data)
    }

    #[test]
    fn test_generic_strategies() {
        let data = vec![5, 2, 9, 1, 5, 6];
        let expected = vec![1, 2, 5, 5, 6, 9];

        // 1. Bubble Sort (Strategy 1)
        let res_bubble = run_strategy(BubbleSorter, &data);
        assert_eq!(res_bubble.sorted_data, expected, "Bubble Sort failed");

        // 2. Quick Sort (Strategy 2)
        let res_quick = run_strategy(QuickSorter, &data);
        assert_eq!(res_quick.sorted_data, expected, "Quick Sort failed");
    }

    #[test]
    fn test_specialized_strategy() {
        // Radix Sort only works for u64, but fits the same interface!
        let data: Vec<u64> = vec![170, 45, 75, 90, 802, 24, 2, 66];
        let expected = vec![2, 24, 45, 66, 75, 90, 170, 802];

        // We can pass RadixSorter to the same generic runner because it implements Sorter<u64>
        let res = run_strategy(RadixSorter, &data);
        assert_eq!(res.sorted_data, expected, "Radix Sort failed");

        // We can ALSO use BubbleSorter for u64!
        let res_bubble = run_strategy(BubbleSorter, &data);
        assert_eq!(res_bubble.sorted_data, expected, "Bubble Sort on u64 failed");
    }

    #[test]
    fn test_interchangeability() {
        // This test proves we can store strategies in a way that allows runtime selection,
        // IF we box them (which incurs dynamic dispatch cost, but proves the trait works).

        // Note: Generic `Sorter<T>` is not object safe if methods have generic parameters or return `Self`.
        // Our `sort` returns `SortingResult<T>` which is fine.
        // `Sorter<T>` should be object safe.

        let data = vec![3, 1, 4, 1, 5];
        let expected = vec![1, 1, 3, 4, 5];

        let sorters: Vec<Box<dyn Sorter<i32>>> = vec![
            Box::new(BubbleSorter),
            Box::new(QuickSorter),
        ];

        for sorter in sorters {
            let res = sorter.sort(&data);
            assert_eq!(res.sorted_data, expected);
        }
    }
}
