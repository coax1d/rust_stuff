struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>
}
pub struct LinkedList<T> {
    head: Option<Node<T>> // Could instead make this Option<Box<Node<T>>> for simplicity
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{ head: None }
    }

    fn push(&mut self, value: T) {
        let old_head = self.head.take();

        let new_head = match old_head {
            None => Node { val: value, next: None },
            Some(old_head) => Node { val: value, next: Some(Box::new(old_head))}
        };

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;

        let new_head = match old_head.next {
            None => None,
            Some(node) => Some(*node)
        };
        self.head = new_head;
        Some(old_head.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = LinkedList::<i32>::new();
        list.push(2);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }
}
