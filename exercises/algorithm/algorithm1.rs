/*
    合并有序链表
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    // 节点
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    // 新建一个节点
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    // 链表
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

// 给 LinkedList 实现 Default trait，可以直接使用 LinkedList::default() 来创建链表
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
        where T: Ord + Copy 
        // 约束范型T是实现了Ord trait的，可比较，并且可在栈上复制（因为我们不能 move，而且下面 tests 给的是 i32）
    {
        // 两个链表的开始节点
        let (mut node_a, mut node_b) = (list_a.start, list_b.start);
        let mut merged_list = LinkedList::default();
        // 如果 node_a 和 node_b 不是 None，那么就会继续循环
        while let (Some(a), Some(b)) = (node_a, node_b) {
            // 通过指针取出元素
            let val_a = unsafe { &(*a.as_ptr()).val };
            let val_b = unsafe { &(*b.as_ptr()).val };
            if val_a <= val_b {
                merged_list.add(*val_a); // 向合并的链表添加元素 val_a
                node_a = unsafe { (*a.as_ptr()).next }; // 更新 node_a
            } else {
                merged_list.add(*val_b); // 向合并的链表添加元素 val_b
                node_b = unsafe { (*b.as_ptr()).next }; // 更新 node_b
            }
        }
        // 如果 list_a 有剩余元素，向 merged_list 添加剩下的元素
        while let Some(a) = node_a {
            let val_a = unsafe { &(*a.as_ptr()).val };
            merged_list.add(*val_a);
            node_a = unsafe { (*a.as_ptr()).next };
        }
        // 如果 list_b 有剩余元素，向 merged_list 添加剩下的元素
        while let Some(b) = node_b {
            let val_b = unsafe { &(*b.as_ptr()).val };
            merged_list.add(*val_b);
            node_b = unsafe { (*b.as_ptr()).next };
        }
        merged_list
    }
}

// 实现 Display trait，便于可以直接 print
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
