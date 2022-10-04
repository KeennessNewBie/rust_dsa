#[derive(Debug)]
pub struct Stack<T> {
    pub top: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct StackNode<T> {
    pub val: T,
    pub next: Option<Box<StackNode<T>>>,
}

// impl `new`, `push`,`pop` for Stack
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack_new() {
        let mut s = Stack::new();
        assert_eq!(s.pop(), None);
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
    }
}
