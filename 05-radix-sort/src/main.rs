fn main() {
    let mut nums = [
        20, 51, 3, 801, 415, 62, 4, 17, 19, 11, 1, 100, 1244, 104, 944, 854, 34, 3000, 3001, 1200,
        633,
    ];
    radix_sort(&mut nums);
    assert_eq!(
        nums,
        [
            1, 3, 4, 11, 17, 19, 20, 34, 51, 62, 100, 104, 415, 633, 801, 854, 944, 1200, 1244,
            3000, 3001
        ]
    );
}

fn radix_sort(items: &mut [usize]) {
    if items.is_empty() {
        return;
    }

    let places = items.iter().max().unwrap().to_string().len();
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); 10];

    for place in 1..=places {
        for &item in items.iter() {
            let item_as_string = item.to_string();
            let digit: usize = if item_as_string.len() < place {
                0
            } else {
                let index = item.to_string().len() - place;
                item_as_string.get(index..=index).unwrap().parse().unwrap()
            };

            buckets[digit].push(item);
        }

        for (i, item) in buckets.concat().into_iter().enumerate() {
            items[i] = item
        }

        buckets = vec![Vec::new(); 10];
    }
}
