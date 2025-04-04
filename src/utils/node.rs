#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
      Node { data, next: None }
    }

    pub fn get_next(&mut self) -> Option<Box<Node<T>>>{
      return self.next.take();
    }
    
    pub fn set_next(&mut self, next: Option<Box<Node<T>>>){
      self.next = next;
    }
}