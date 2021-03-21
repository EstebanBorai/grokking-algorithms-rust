/// Serves a collection of elements keeping the order of arrival.
/// Opposite to Stacks, a Queue is a FIFO datastructutre, which
/// means that the first element inserted will be the first element
/// to go out of the Queue.
///
/// This implementation of a Queue datastructure is naive and doesn't
/// achieves good performance. For real life situations a `VecDeque`
/// from `std::collections` is a better choice.
#[allow(dead_code)]
struct Queue<T> {
    coll: Vec<T>,
}

impl<T> Queue<T> {
    /// Creates a new instance of an empty Queue
    fn new() -> Self {
        Queue { coll: Vec::new() }
    }

    /// Appends an element to the Queue
    fn enqueue(&mut self, item: T) {
        self.coll.push(item);
    }

    /// Takes the next element from the Queue
    fn dequeue(&mut self) -> T {
        self.coll.remove(0)
    }

    /// Retrieves the number of elements conforming the Queue
    fn length(&self) -> usize {
        self.coll.len()
    }

    /// Peeks the next element from the Queue.
    /// This is similar to calling `dequeue` but
    /// without moving the value from the Queue
    fn peek(&self) -> Option<&T> {
        self.coll.get(0)
    }

    /// Returns `true` if the Queue is empty otherwise
    /// returns `false`
    fn is_empty(&self) -> bool {
        self.coll.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_queue() {
        let empty_vec: Vec<i32> = Vec::new();
        let queue: Queue<i32> = Queue::new();

        assert_eq!(queue.coll, empty_vec);
    }

    #[test]
    fn enqueue_an_element() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(9);

        assert_eq!(queue.coll, vec![9]);
    }

    #[test]
    fn dequeues_an_element() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(9);
        queue.enqueue(10);
        queue.enqueue(11);
        queue.enqueue(12);

        assert_eq!(queue.dequeue(), 9);
        assert_eq!(queue.dequeue(), 10);
        assert_eq!(queue.dequeue(), 11);
        assert_eq!(queue.dequeue(), 12);
    }

    #[test]
    fn retrieves_queue_element_count() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(9);
        queue.enqueue(10);
        queue.enqueue(11);
        queue.enqueue(12);

        assert_eq!(queue.length(), 4_usize);
    }

    #[test]
    fn peeks_the_next_element_from_the_queue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(9);
        queue.enqueue(10);
        queue.enqueue(11);
        queue.enqueue(12);

        assert_eq!(queue.peek(), Some(&9));
    }

    #[test]
    fn checks_if_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();

        assert!(queue.is_empty());
    }

    #[test]
    fn checks_if_queue_is_not_empty() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(9);
        queue.enqueue(10);
        queue.enqueue(11);
        queue.enqueue(12);

        assert!(!queue.is_empty());
    }
}
