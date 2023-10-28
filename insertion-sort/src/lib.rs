pub fn insertion_sort(array: &mut Vec<i32>) {
    for i in 2..=array.len() {
        let key = array[i - 1];

        let mut j = i - 1;
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1];
            j -= 1;
        }

        array[j] = key;
    }
}

pub fn r_insertion_sort(array: &mut Vec<i32>) {
    for i in 2..=array.len() {
        let key = array[i - 1];

        let mut j = i - 1;
        while j > 0 && array[j - 1] < key {
            array[j] = array[j - 1];
            j -= 1;
        }

        array[j] = key;
    }
}

pub fn linear_search(array: &[i32], number: i32) -> Option<usize> {
    for (index, &value) in array.iter().enumerate() {
        if value == number {
            return Some(index);
        }
    }

    None
}
