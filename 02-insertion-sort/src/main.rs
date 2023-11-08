fn insertion_sort<T: Ord>(items: &mut [T]) {
    for i in 1..items.len() {
        for j in (0..i).rev() {
            let is_greater = items[j] > items[j + 1];

            if !is_greater {
                break;
            }

            items.swap(j, j + 1);
        }
    }
}

fn r_insertion_sort<T: Ord>(items: &mut [T]) {
    for i in 1..items.len() {
        for j in (0..i).rev() {
            let is_lower = items[j] < items[j + 1];

            if !is_lower {
                break;
            }

            items.swap(j, j + 1);
        }
    }
}

fn main() {
    let mut nums = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];

    insertion_sort(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    r_insertion_sort(&mut nums);
    assert_eq!(nums, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
}
