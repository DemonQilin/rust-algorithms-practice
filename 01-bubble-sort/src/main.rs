fn bubble_sort<T: Ord>(items: &mut [T]) {
    loop {
        let mut swapped = false;

        for i in 0..(items.len() - 1) {
            let is_greather = items[i] > items[i + 1];

            if is_greather {
                items.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut nums = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
    bubble_sort(&mut nums);

    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
