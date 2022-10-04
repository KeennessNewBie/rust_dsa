#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, val: T) {
        self.elements.push(val);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("queue is empty")
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(v) => Ok(v),
            None => Err("queue is empty"),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(64);
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_dequeue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(32);
        queue.enqueue(64);
        let retrieved_dequeue = queue.dequeue();
        assert!(retrieved_dequeue.is_ok());
        assert_eq!(32, retrieved_dequeue.unwrap());
    }

    #[test]
    fn test_peek() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        let retrieved_peek = queue.peek();
        assert!(retrieved_peek.is_ok());
        assert_eq!(8, *retrieved_peek.unwrap());
    }

    #[test]
    fn test_size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        assert_eq!(2, queue.size());
    }
}
