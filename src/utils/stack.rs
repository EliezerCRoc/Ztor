use crate::utils::node::Node;
use std::mem;

#[derive(Debug)]
pub struct Stack<T> {
    pub top: Option<Node<T>>,
}

impl<T> Stack<T> {

    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn is_empty(&self) -> bool {
      match self.top {
        Some(_) => false,
        None => true,
      }
    }

    pub fn push(&mut self, data: T) {
        let mut node = Node::new(data);
        if let Some(top) = std::mem::replace(&mut self.top, None) {
          node.set_next(Some(Box::new(top)));          
        }
        self.top = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut top) = mem::replace(&mut self.top, None) {
          self.top = match top.get_next() {
            Some(n) => Some(*n), 
            None => None,
          };
          Some(top.data)
        } else {
          None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.top {
          Some(top) => Some(&top.data),
          None => None,
        }
    }

}
