/*
    堆
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    // 比较闭包函数，根据该函数来确定谁在堆顶
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
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

    pub fn add(&mut self, value: T) {
        self.items.push(value); // 插入到树的最后面
        self.count += 1;
        self.sift_up(self.count);
    }

    // 上滤操作，将新插入的元素上浮到合适的位置
    fn sift_up(&mut self, mut idx: usize) {
        // 保证 idx 不是根节点，比较当前节点以及父节点
        while idx > 1 && ((self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)])) {
            // 比较成功
            let parent_idx = self.parent_idx(idx);
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }
    // 弹出堆顶
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let top = self.items[1].clone();
        // 交换第一个和最后一个
        self.items.swap(1, self.count);
        // 移除最后一个
        self.items.pop();
        self.count -= 1;
        // 下滤操作
        if !self.is_empty() {
            self.sift_down(1); // 调整堆的结构
        }
        Some(top)
    }

    fn sift_down(&mut self, mut idx: usize) {
        // 当当前节点有孩子节点时
        while self.children_present(idx) {
            // 交换子节点中较小的
            let smallest = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest], &self.items[idx]) {
                self.items.swap(smallest, idx);
                idx = smallest;
            } else {
                break;
            }
        }
    }

    // 父节点，当前节点 / 2
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // 用来找该节点是否有孩子节点
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
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && ((self.comparator)(&self.items[right], &self.items[left])) {
            right
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap 小根堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap 大根堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Clone>() -> Heap<T>
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
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

fn main() {}

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