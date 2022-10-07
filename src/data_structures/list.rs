use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    val: T,
    prev: Option<Rc<RefCell<ListNode<T>>>>,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> ListNode<T> {
        ListNode {
            val,
            prev: None,
            next: None,
        }
    }
}

impl<T: std::fmt::Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.val)
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    length: usize,
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = self.length;
        write!(f, "[")?;
        for i in 0..length {
            match self.get_index_node(i) {
                None => (),
                Some(e) => write!(f, "{}\t", e.borrow().val)?,
            }
        }
        write!(f, "]")
    }
}
// TODO add fn has some problem in cycle

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn add_first(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        match self.head {
            None => {
                self.tail.replace(new_node.clone());
                self.head = Some(new_node);
            }
            Some(ref head) => {
                new_node.borrow_mut().next.replace(head.clone());
                head.borrow_mut().prev.replace(new_node.clone());
                self.head.replace(new_node);
            }
        }
        self.length += 1;
    }

    pub fn add_last(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        match self.tail {
            None => {
                self.head.replace(new_node.clone());
                self.tail.replace(new_node);
            }
            Some(ref tail) => {
                new_node.borrow_mut().prev.replace(tail.clone());
                tail.borrow_mut().next.replace(new_node.clone());
                self.tail.replace(new_node);
            }
        }
        self.length += 1;
    }

    pub fn get_index_node(&self, index: usize) -> Option<Rc<RefCell<ListNode<T>>>> {
        match self.get_index_node_by_node(self.head.clone(), index) {
            None => None,
            Some(e) => Some(e),
        }
    }

    // -> Rc<RefCell<ListNode<T>>>
    pub(self) fn get_index_node_by_node(
        &self,
        node: Option<Rc<RefCell<ListNode<T>>>>,
        index: usize,
    ) -> Option<Rc<RefCell<ListNode<T>>>> {
        // if let Some(cur_node) = node.clone() {
        //     let a = self.get_index_node(cur_node.clone(), index);
        //     return a.clone();
        // }
        match node {
            None => None,
            Some(cur_node) => match index {
                0 => Some(cur_node.clone()),
                _ => self
                    .get_index_node_by_node(cur_node.clone().borrow_mut().next.clone(), index - 1),
            },
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add_first() {
        let mut x = LinkedList::new();
        x.add_first(1);
        x.add_first(2);
    }
}
