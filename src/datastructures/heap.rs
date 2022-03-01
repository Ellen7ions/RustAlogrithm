pub struct Heap<T> {
    data: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(f: fn(&T, &T) -> bool) -> Heap<T> {
        Heap {
            data: Vec::new(),
            comparator: f,
        }
    }

    fn father(pos: usize) -> usize {
        (pos - 1) >> 1
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        self.shift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.data.len();
        self.data.swap(0, len - 1);
        let res = self.data.pop();
        self.shift_down(0);
        res
    }

    pub fn peek(&self) -> &T {
        &self.data[0]
    }

    fn shift_up(&mut self, pos: usize) {
        let mut p = pos;
        while p > 0 && (self.comparator)(&self.data[p], &self.data[Self::father(p)]) {
            self.data.swap(p, Self::father(p));
            p = Self::father(p);
        }
    }

    fn shift_down(&mut self, pos: usize) {
        let len = self.data.len();
        let mut p = pos;
        let mut lch = (p << 1) + 1;
        while lch < len {
            let tmp = if lch + 1 < len && (self.comparator)(&self.data[lch + 1], &self.data[lch]) {
                lch + 1
            } else {
                lch
            };
            if (self.comparator)(&self.data[tmp], &self.data[p]) {
                self.data.swap(p, tmp);
            }
            p = tmp;
            lch = (p << 1) + 1;
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Heap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

pub fn get_max_heap<T: Ord>() -> Heap<T> {
    Heap::new(|x, y| x > y)
}

pub fn get_min_heap<T: Ord>() -> Heap<T> {
    Heap::new(|x, y| x < y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heap() {
        let mut min_heap: Heap<i32> = get_min_heap();
        min_heap.push(5);
        min_heap.push(2);
        min_heap.push(8);
        min_heap.push(1);
        min_heap.push(5);
        min_heap.push(7);
        println!("{}", min_heap);
        // println!("pop => {}", min_heap.pop().expect("None"));
        assert_eq!(min_heap.pop().expect("None"), 1);
        println!("{}", min_heap);
        assert_eq!(min_heap.pop().expect("None"), 2);
        println!("{}", min_heap);
        assert_eq!(min_heap.pop().expect("None"), 5);
        println!("{}", min_heap);
        assert_eq!(min_heap.pop().expect("None"), 5);
        println!("{}", min_heap);
        assert_eq!(min_heap.pop().expect("None"), 7);
        println!("{}", min_heap);
        assert_eq!(min_heap.pop().expect("None"), 8);
        println!("{}", min_heap);
    }
}
