/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT 

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bubble_up(&mut self, idx: usize) {
        let mut current = idx;
        while current > 1 {
            let parent = self.parent_idx(current);
            if (self.comparator)(&self.items[current], &self.items[parent]) {
                self.items.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }
    pub fn bubble_down(&mut self, idx: usize) {
        let mut current = idx;
        while self.children_present(current) {
            let smallest_child = self.smallest_child_idx(current);
            if (self.comparator)(&self.items[smallest_child], &self.items[current]) {
                self.items.swap(smallest_child, current);
                current = smallest_child;
            } else {
                break;
            }
        }
    }
    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.items.len() <= self.count {
            self.items.push(T::default());
        }
        self.items[self.count] = value;
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child = self.left_child_idx(idx);
        let right_child = self.right_child_idx(idx);

        if right_child > self.count {
            return left_child;
        }

        if (self.comparator)(&self.items[left_child], &self.items[right_child]) {
            left_child
        } else {
            right_child
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let mut root: T = T::default();
            std::mem::swap(&mut root, &mut self.items[1]);
            self.items.swap(1, self.count);
            self.count -= 1;
            self.bubble_down(1);
            Some(root)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}