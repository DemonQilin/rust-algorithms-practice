pub fn merge_sort_without_cloning<T: Ord + Clone>(items: &mut [T]) {
    if items.is_empty() || items.len() == 1 {
        return;
    }

    let middle = items.len() / 2;

    merge_sort_without_cloning(&mut items[..middle]);
    merge_sort_without_cloning(&mut items[middle..]);

    merge_without_cloning(items, middle)
}

fn merge_without_cloning<T: Ord>(items: &mut [T], middle: usize) {
    if middle >= items.len() {
        panic!(
            "lower out of bounds: the len is {} but the middle is {middle}",
            items.len()
        )
    }

    let mut position_of_initials = (0..items.len()).collect::<Vec<_>>();
    let mut positions_in_terms_of_initials = position_of_initials.clone();

    let mut i = 0;
    let mut j = middle;
    let mut k = 0;

    while i < middle && j < items.len() {
        let left = &items[position_of_initials[i]];
        let right = &items[position_of_initials[j]];
        let initial_index = if left <= right { &mut i } else { &mut j };

        fill_index(
            items,
            &mut position_of_initials,
            &mut positions_in_terms_of_initials,
            k,
            *initial_index,
        );

        *initial_index += 1;
        k += 1;
    }

    while i < middle {
        fill_index(
            items,
            &mut position_of_initials,
            &mut positions_in_terms_of_initials,
            k,
            i,
        );

        i += 1;
        k += 1;
    }

    while j < items.len() {
        fill_index(
            items,
            &mut position_of_initials,
            &mut positions_in_terms_of_initials,
            k,
            j,
        );

        j += 1;
        k += 1;
    }
}

fn fill_index<'a, T>(
    items: &'a mut [T],
    positions_of_initials: &'a mut [usize],
    positions_in_initials_terms: &'a mut [usize],
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
