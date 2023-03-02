use std::collections::VecDeque;

/// Homemade implementation of a stack, because no course involving programming exercises is
/// complete without such a thing.
///
/// All actual work is delegated to Rust's VecDeque type.
pub struct Stack<T> {
    content: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            content: VecDeque::new(),
        }
    }

    /// Return whether stack is empty.
    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }

    /// Return number of items contained in stack.
    pub fn size(&self) -> usize {
        // Deque keeps track of its own size, so this is O(1)
        self.content.len()
    }

    /// Push a new item onto the stack.
    pub fn push(&mut self, item: T) {
        self.content.push_front(item);
    }

    /// Return the item on top of the stack (if any) without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.content.front()
    }

    /// Remove and return the item on top of the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.content.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack: Stack<u32> = Stack::new();
        stack.push(2);
        stack.push(3);
        stack.push(5);

        assert_eq!(stack.pop().unwrap(), 5);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);

        // Pop from empty stack
        assert!(stack.pop().is_none());
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<u32> = Stack::new();
        stack.push(2);
        stack.push(3);

        // Peeking should not remove the item
        assert_eq!(*stack.peek().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 3);

        assert_eq!(*stack.peek().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 2);

        // Peek onto empty stack
        assert!(stack.peek().is_none());
    }

    #[test]
    fn test_size_and_is_empty() {
        let mut stack: Stack<u32> = Stack::new();
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());

        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 2);
        assert!(!stack.is_empty());

        stack.pop();
        stack.pop();

        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }
}
