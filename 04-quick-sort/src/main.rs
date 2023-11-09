fn main() {
    let mut nums = [10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
    quick_sort_with_cloning(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    nums = [5, 3, 18, 21, 12, 1, 5, 1, 54, 6];
    quick_sort_without_cloning(&mut nums);
    assert_eq!(nums, [1, 1, 3, 5, 5, 6, 12, 18, 21, 54]);
}

fn quick_sort_with_cloning<T: Ord + Clone>(items: &mut [T]) {
    if items.len() <= 1 {
        return;
    }

    let pivot = items.last().unwrap().clone();
    let mut left = vec![];
    let mut right = vec![];

    for item in &items[..items.len() - 1] {
        if item <= &pivot {
            left.push(item.clone());
        } else {
            right.push(item.clone());
        };
    }

    quick_sort_with_cloning(&mut left);
    quick_sort_with_cloning(&mut right);

    let result = [left, vec![pivot], right].concat();

    for (i, item) in result.into_iter().enumerate() {
        items[i] = item
    }
}

fn quick_sort_without_cloning<T: Ord>(items: &mut [T]) {
    if items.len() <= 1 {
        return;
    }

    let mut positions_of_initials: Vec<usize> = (0..items.len()).collect();
    let mut positions_in_initials_terms = positions_of_initials.clone();

    let [mut total_greater, mut total_less] = [0usize, 0];

    for i in 0..(items.len() - 1) {
        let pivot = &items[*positions_of_initials.last().unwrap()];
        let item = &items[positions_of_initials[i]];

        if item <= pivot {
            fill_index(
                items,
                &mut positions_of_initials,
                &mut positions_in_initials_terms,
                total_less,
                i,
            );

            total_less += 1;
        } else {
            let index_to_fill = items.len() - total_greater - 2;

            fill_index(
                items,
                &mut positions_of_initials,
                &mut positions_in_initials_terms,
                index_to_fill,
                i,
            );

            total_greater += 1;
        }
    }

    quick_sort_without_cloning(&mut items[..total_less]);
    quick_sort_without_cloning(&mut items[total_less..total_less + total_greater]);

    for i in 1..=total_greater {
        let current_pivot_index = items.len() - i;
        let next_pivot_index = items.len() - i - 1;
        items.swap(current_pivot_index, next_pivot_index);
    }
}

fn fill_index<T>(
    items: &mut [T],
    positions_of_initials: &mut [usize],
    positions_in_initials_terms: &mut [usize],
    index_to_fill: usize,
    initial_index_of_element: usize,
) {
    let current_index_in_items = positions_of_initials[initial_index_of_element];
    let initial_index_in_fill = positions_in_initials_terms[index_to_fill];

    items.swap(index_to_fill, current_index_in_items);

    positions_of_initials[initial_index_in_fill] = current_index_in_items;
    positions_of_initials[initial_index_of_element] = index_to_fill;
    positions_in_initials_terms[current_index_in_items] = initial_index_in_fill;
    positions_in_initials_terms[index_to_fill] = initial_index_of_element;
}
