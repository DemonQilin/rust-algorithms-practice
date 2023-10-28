pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn find(&mut self, index: usize) -> Option<&mut Node<T>> {
        if self.length == 0 {
            return None;
        }

        if index >= self.length {
            return None;
        }

        let mut node = self.head.as_mut();

        for _ in 0..index {
            if node.is_some() {
                let current_node = node.unwrap();

                node = if current_node.next.is_some() {
                    Some(current_node.next.as_mut().unwrap().as_mut())
                } else {
                    None
                }
            } else {
                return None;
            }
        }

        node
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.length == 0 {
            return None;
        }

        if index >= self.length {
            return None;
        }

        let mut node = self.head.as_ref();

        for _ in 0..index {
            if node.is_some() {
                let current_node = node.unwrap();

                node = if current_node.next.is_some() {
                    Some(current_node.next.as_ref().unwrap().as_ref())
                } else {
                    None
                }
            } else {
                return None;
            }
        }

        node.map(|node| &node.value)
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node { value, next: None };

        if self.length == 0 {
            self.head = Some(new_node);
        } else {
            let last_node = self.find(self.length - 1).unwrap();
            last_node.next = Some(Box::new(new_node))
        }

        self.length += 1
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        if self.is_empty() {
            return None;
        }

        if index == 0 || self.length == 1 {
            let mut node = self.head.take();
            let next = node.as_mut().unwrap().next.take().map(|node| *node);
            let value = node.map(|node| node.value);

            self.head = next;
            self.length -= 1;

            return value;
        }

        let before_node = self.find(index - 1).unwrap();
        let value: Option<T>;

        if let Some(target_node) = before_node.next.take() {
            before_node.next = target_node.next;
            value = Some(target_node.value);
        } else {
            before_node.next = None;
            value = None;
        }

        self.length -= 1;

        value
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.delete(self.length - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn abc_range(length: u32) -> Vec<char> {
        let mut vec = Vec::new();

        for i in 0..length {
            vec.push(char::from_u32(i + 97).unwrap())
        }

        vec
    }

    #[test]
    fn push_tests() {
        let mut list = LinkedList::new();
        abc_range(26).into_iter().for_each(|char| list.push(char));

        assert_eq!(list.len(), 26);
    }

    #[test]
    fn pop_tests() {
        let mut list = LinkedList::new();
        abc_range(13).into_iter().for_each(|char| list.push(char));
        assert_eq!(list.len(), 13);

        for _ in 0..10 {
            list.pop();
        }

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some('c'));
    }

    #[test]
    fn get_tests() {
        let mut list = LinkedList::new();
        list.push("first".to_string());
        assert_eq!(list.get(0), Some(&"first".to_string()));

        list.push("second".to_string());
        assert_eq!(list.get(1), Some(&"second".to_string()));
        assert_eq!(list.get(0), Some(&"first".to_string()));

        abc_range(26)
            .into_iter()
            .for_each(|char| list.push(char.to_string()));
        assert_eq!(list.get(27), Some(&"z".to_string()));
        assert_eq!(list.get(0), Some(&"first".to_string()));
        assert_eq!(list.get(9), Some(&"h".to_string()));
    }

    #[test]
    fn delete_tests() {
        let mut list = LinkedList::new();
        abc_range(26).into_iter().for_each(|char| list.push(char));
        list.delete(13);
        assert_eq!(list.len(), 25);
        assert_eq!(list.get(12), Some(&'m'));
        assert_eq!(list.get(13), Some(&'o'));

        list.delete(0);
        assert_eq!(list.len(), 24);
        assert_eq!(list.get(0), Some(&'b'));
    }
}
