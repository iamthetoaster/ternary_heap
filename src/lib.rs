//! A priority queue implemented with a ternary heap.
//! 


#[derive(Debug, Clone)]
pub struct TernaryHeap<T> {
    data: Vec<T>,
}

impl <T: Ord> TernaryHeap<T> {
    
    /// Creates a new `TernaryHeap` as a max-heap.
    pub fn new() -> Self {
        TernaryHeap{ data: vec![] }
    }

    /// Creates a new `TernaryHeap` with a specified capacity.
    /// This preallocates enough space for `capacity` elements,
    /// so the internal `Vec` doesn't need to reallocate until 
    /// the heap contains that many values.
    pub fn with_capacity(capacity: usize) -> Self {
        TernaryHeap{ data: Vec::with_capacity(capacity) }
    }

    /// Removes the greatest value from the heap and returns it, 
    /// or `None` if the heap is empty.
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            let last = self.len() - 1;
            self.data.swap(0, last);

            self.sink_until(0, last);
        }
        self.data.pop()
    }

    /// Adds a value to the heap.
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.swim(old_len);
    }

    /// Returns the number of items stored in the heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn swim(&mut self, pos: usize) {
        let mut pos = pos;
        while pos > 0 {
            let parent = Self::parent(pos);
            if self.data[pos] <= self.data[parent] {
                return;
            }
            self.data.swap(pos, parent);
            pos = parent;
        }
    }

    fn sink_until(&mut self, pos: usize, end: usize) {
        let mut pos = pos;
        while let Some(child) = self.best_child(pos, end) {
            if self.data[pos] >= self.data[child] {
                return;
            }
            self.data.swap(pos, child);
            pos = child;
        }
    }

    fn sink(&mut self, pos: usize) {
        self.sink_until(pos, self.data.len())
    }

    fn best_child(&self, parent: usize, end: usize) -> Option<usize> {
        match Self::children(parent, end) {
            Some(vec) => vec.into_iter().max_by_key(|i| &self.data[*i]),
            None => None
        }
    }

    fn parent(child: usize) -> usize {
        if child == 0 {
            0
        } else {
            (child - 1) / 3
        }
    }

    fn children(parent: usize, end: usize) -> Option<Vec<usize>> {
        let first_child = parent * 3 + 1;
        if first_child >= end {
            return None
        }
        let last_child = (first_child + 3).min(end);
        let result = (first_child..last_child).collect();
        return Some(result);
    }
}

impl<T: Ord> From<Vec<T>> for TernaryHeap<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut heap = TernaryHeap{ data: vec };
        let last_parent = Self::parent(heap.len() - 1);
        for parent in (0..=last_parent).rev() {
            heap.sink(parent);
        }
        heap
    }
}

impl<T> From<TernaryHeap<T>> for Vec<T> {
    fn from(heap: TernaryHeap<T>) -> Self {
        heap.data
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;
    use rand::{Rng, SeedableRng, seq::SliceRandom};
    use rand::rngs::SmallRng;

    const TEST_SIZE: u64 = 1000;
    const TEST_ITERATIONS: u64 = 1000;

    #[test]
    fn single_test_against_binary() {
        specific_test_against_binary(10)
    }

    #[test]
    #[ignore]
    fn many_test_against_binary() {
        for seed in 0..TEST_ITERATIONS {
            specific_test_against_binary(seed);
        }
    }

    fn specific_test_against_binary(seed: u64) {
        let mut rand = SmallRng::seed_from_u64(seed);
        let mut vec: Vec<_> = (0..TEST_SIZE).collect();
        vec.shuffle(&mut rand);

        let mut binary = BinaryHeap::new();
        let mut trnary = TernaryHeap::new();
        
        while !vec.is_empty() {
            if rand.gen() {
                let value = vec.pop().unwrap();
                binary.push(value);
                trnary.push(value);
            } else {
                assert_eq!(binary.pop(), trnary.pop());
            }
        }

        while !binary.is_empty() {
            assert_eq!(binary.pop(), trnary.pop());
        }

        assert!(trnary.is_empty());
    }

    #[test]
    fn single_test_from_vec() {
        specific_test_from_vec(10);
    }

    #[test]
    #[ignore]
    fn many_test_from_vec() {
        for i in 0..TEST_ITERATIONS {
            specific_test_from_vec(i);
        }
    }

    fn specific_test_from_vec(seed: u64) {
        let mut rand = SmallRng::seed_from_u64(seed);
        let mut vec: Vec<_> = (0..TEST_SIZE).collect();
        vec.shuffle(&mut rand);

        let mut heap: TernaryHeap<_> = vec.into();
        heap.verify_heap();

        let mut last = heap.pop();
        while !heap.is_empty() {
            let next = heap.pop();
            assert!(last >= next);
            last = next;
        }
    }


    impl<T: Ord + std::fmt::Debug> TernaryHeap<T> {
        fn verify_heap(&self) {
            for (i, val) in self.data.iter().enumerate() {
                if let Some(children) = Self::children(i, self.len()) {
                    for child_index in children {
                        assert!(
                            val >= &self.data[child_index], 
                            "Heap condition broken between indices {} (value: {:?}) and {} (value: {:?})\n{:?})", 
                            child_index, &self.data[child_index], i, val, self.data
                        );
                    }
                }
            }
        }
    }


    #[derive(Debug)]
    pub struct HeapConditionError {
        heap: String,
        indices: Vec<(usize, usize)>,
    }
}
