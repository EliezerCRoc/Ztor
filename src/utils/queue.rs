use std::collections::VecDeque;

#[derive(Debug)]

pub struct Queue<T>{
    data: VecDeque<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: VecDeque::new() }
    }
    
    pub fn enqueue(&mut self, item: T) {
        self.data.push_back(item);
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty(){
            return None;
        }
        self.data.pop_front()
    }
    
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty(){
            return None;
        }
        self.data.front()
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}