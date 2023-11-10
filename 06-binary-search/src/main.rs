use std::cmp::Ordering;

fn main() {
    let users = [
        User::new(1, "Sam"),
        User::new(3, "Sarah"),
        User::new(5, "John"),
        User::new(6, "Burke"),
        User::new(10, "Simona"),
        User::new(12, "Asim"),
        User::new(13, "Niki"),
        User::new(15, "Aysegul"),
        User::new(17, "Kyle"),
        User::new(18, "Jem"),
        User::new(19, "Marc"),
        User::new(21, "Chris"),
        User::new(23, "Juanes"),
        User::new(24, "Ben"),
    ];
    let target_user = User::new(23, "Juanes");

    assert_eq!(binary_search(&target_user, &users), Some(12));
}

fn binary_search<T: Ord>(target: &T, items: &[T]) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let mut min = 0usize;
    let mut max = items.len() - 1;
    let mut middle = (max + min) / 2;
    let mut item = &items[middle];

    while target != item {
        if middle == min || middle == max {
            return None;
        }

        if target < item {
            max = middle
        } else {
            min = middle
        };

        middle = (max + min) / 2;
        item = &items[middle];
    }

    Some(middle)
}

struct User {
    id: u32,
    name: String,
}

impl User {
    fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
