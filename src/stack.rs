/// Serves a collection of elements keeping the order of arrival.
/// (LIFO)
///
/// The latest element allocated will be the first to be extracted.
///
/// This implementation of the stack uses a `Vec` under the hood and
/// limits its methods (by exposing Stack only methods) to achieve
/// the LIFO behavior.
#[allow(dead_code)]
struct Stack<T> {
    coll: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new instance of an empty Stack
    pub fn new() -> Self {
        Stack { coll: Vec::new() }
    }

    /// Pushes an element on top of the others
    pub fn push(&mut self, element: T) {
        self.coll.push(element);
    }

    /// Pops the topmost element from the Stack and returns it.
    /// If no element is available (The stack is empty) returns
    /// `None` instead
    pub fn pop(&mut self) -> Option<T> {
        self.coll.pop()
    }

    /// Retrieves the count of element conforming the stack
    pub fn length(&self) -> usize {
        self.coll.len()
    }

    /// Retrieves a reference to the last element inserted.
    /// This is similar to calling `pop` but doesn't moves
    /// the element from the Stack
    pub fn peek(&self) -> Option<&T> {
        self.coll.get(self.length() - 1)
    }

    /// Returns `true` if the Stack is empty, otherwise returns `false`
    pub fn is_empty(&self) -> bool {
        self.coll.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_stack() {
        let empty_vec: Vec<i32> = Vec::new();
        let stack: Stack<i32> = Stack::new();

        assert_eq!(stack.coll, empty_vec);
    }

    #[test]
    fn pushes_an_element_into_the_stack() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(10);

        let mut stack: Stack<i32> = Stack::new();
        stack.push(10);

        assert_eq!(stack.coll, vec);
        assert_eq!(stack.length(), 1);
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn pops_elements_from_the_stack() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(10);
        stack.push(9);
        stack.push(8);
        stack.push(7);
        stack.push(6);
        stack.push(5);

        assert_eq!(stack.length(), 6);
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.length(), 5);
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.length(), 4);
        assert_eq!(stack.pop(), Some(7));
    }

    #[test]
    fn returns_stack_length() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        assert_eq!(stack.length(), 5);
    }

    #[test]
    fn peeks_element_from_stack() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        assert_eq!(stack.peek(), Some(&5));
    }

    #[test]
    fn checks_if_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();

        assert!(stack.is_empty());
    }

    #[test]
    fn checks_if_stack_is_not_empty() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(10);

        assert!(!stack.is_empty());
    }
}
