/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        println!("add: {:?}", value);
        self.count += 1;
        self.items.push(value);
        self.heapify_up(self.count);
        self.print("add");
    }

    fn heapify_up(&mut self, idx: usize) {
        let parent_idx = self.parent_idx(idx);
        if parent_idx > 0 && (self.comparator)(&self.items[idx -1 ], &self.items[parent_idx -1]) {
            self.items.swap(idx -1 , parent_idx -1);
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        self.print("heapify_down");
        match self.smallest_child_idx(idx) {
            Some(smallest_child_idx) => {
                if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                    self.items.swap(smallest_child_idx, idx);
                    self.heapify_down(smallest_child_idx);
                }
            }
            None => {}
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count -1
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        if self.children_present(idx) {
            let left = self.left_child_idx(idx);
            let right = self.right_child_idx(idx);
            println!("index: {}, left: {}, right: {}", idx, left, right);
            if (self.comparator)(&self.items[left], &self.items[right]) {
                Some(left)
            } else {
                Some(right)
            }
        } else {
            None
        }
    }

    fn print(&self, key: &str) {
        println!("{}: count:{}, heap: {:?}", key, self.count, self.items);
    }
}

impl<T> Heap<T>
where
    T: Default + Debug + Ord,
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
    T: Default + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.items.swap(0, self.count - 1);
        let root = self.items.swap_remove(self.count-1);
        self.count -= 1;
        if self.count > 0 {
            self.print("next before heapify_down");
            self.heapify_down(0);
            self.print("next after heapify_down");
        }

        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Debug + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Debug + Ord,
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
        heap.print("test_min_heap");
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        heap.print("test_min_heap");
        assert_eq!(heap.next(), Some(4));
        heap.print("test_min_heap");
        assert_eq!(heap.next(), Some(9));
        heap.print("test_min_heap");
        heap.add(1);
        heap.print("test_min_heap");
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
