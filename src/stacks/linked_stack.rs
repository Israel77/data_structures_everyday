use crate::stacks::stack::Stack;

type NodeLink<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: NodeLink<T>,
}

pub struct LinkedStack<T> {
    top: NodeLink<T>,
    size: usize,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        Self { top: None, size: 0 }
    }
}

impl<T> Stack<T> for LinkedStack<T> {
    fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.top.take(),
        };

        self.top = Some(Box::from(new_node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|current_node| {
            self.top = current_node.next;
            self.size -= 1;
            current_node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn test_is_empty() {
        let mut stack = LinkedStack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_size() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.size(), 0);
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert_eq!(stack.size(), 1);
    }
}
