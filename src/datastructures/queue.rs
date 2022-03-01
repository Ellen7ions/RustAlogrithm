use std::collections::LinkedList;

pub struct Queue<T> {
    queue: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            queue: LinkedList::new(),
        }
    }

    pub fn enq(&mut self, v: T) {
        self.queue.push_back(v);
    }

    pub fn deq(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue() {
        let mut q: Queue<i32> = Queue::new();
        q.enq(1);
        q.enq(2);
        q.enq(3);
        assert_eq!(q.deq(), Some(1));
        assert_eq!(q.len(), 2);
    }
}
