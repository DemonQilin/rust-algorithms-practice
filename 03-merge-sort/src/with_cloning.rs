pub fn merge_sort_with_cloning<T: Ord + Clone>(items: &mut [T]) {
    if items.is_empty() || items.len() == 1 {
        return;
    }

    let middle = items.len() / 2;

    merge_sort_with_cloning(&mut items[..middle]);
    merge_sort_with_cloning(&mut items[middle..]);

    merge_with_cloning(items, middle)
}

fn merge_with_cloning<T: Ord + Clone>(items: &mut [T], middle: usize) {
    if middle >= items.len() {
        panic!(
            "lower out of bounds: the len is {} but the middle is {middle}",
            items.len()
        )
    }

    let mut left: Vec<T> = items[..middle].iter().cloned().rev().collect();
    let mut right: Vec<T> = items[middle..].iter().cloned().rev().collect();

    let mut k = 0;

    while !left.is_empty() && !right.is_empty() {
        let item_to_insert = if left.last() <= right.last() {
            left.pop().unwrap()
        } else {
            right.pop().unwrap()
        };

        items[k] = item_to_insert;
        k += 1;
    }

    let remaining = if left.is_empty() { right } else { left };

    for item in remaining.into_iter().rev() {
        items[k] = item;
        k += 1;
    }
}
