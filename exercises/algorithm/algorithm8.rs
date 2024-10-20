/*
    队列
    queue
    用两个队列来实现栈
    This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
    // 入队，放到最后面
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }
    // 出队，移除最前面的元素
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
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

pub struct myStack<T> {
    sign: bool, // true 表示元素都在 q1 队列，false 表示元素都在 q2 队列
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            sign: true, // 初始为 true，表示 q1
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        match self.sign {
            true => self.q1.enqueue(elem),
            false => self.q2.enqueue(elem),
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        match self.sign {
            true => {
                // 把 q1 前 n-1 个元素移动到 q2
                while self.q1.size() > 1 {
                    let value = self.q1.dequeue().unwrap();
                    self.q2.enqueue(value);
                }
                // 交换，剩下的 q1 的最后一个元素就是栈顶
                self.sign = false;
                self.q1.dequeue()
            }
            false => {
                while self.q2.size() > 1 {
                    let value = self.q2.dequeue().unwrap();
                    self.q1.enqueue(value);
                }
                self.sign = true;
                self.q2.dequeue()
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
