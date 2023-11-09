fn main() {
    let mut nums = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
    merge_sort::merge_sort_with_cloning(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    nums = [5, 3, 18, 21, 12, 1, 5, 1, 54, 6];
    merge_sort::merge_sort_without_cloning(&mut nums);
    assert_eq!(nums, [1, 1, 3, 5, 5, 6, 12, 18, 21, 54])
}
