use std::fmt::Debug;

pub struct SimpleLinkedList<T: Debug>(Option<Box<Node<T>>>);

struct Node<T: Debug> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> where T: Debug {
    pub fn new() -> Self {
        SimpleLinkedList(None)
    }
    
    pub fn printlist(&self) {
        let mut last_node = &self.0;
        loop {
            match last_node {
                Some(node) => {
                    println!("Value stored is {:?}", (*node).data);
                    last_node = &(node).next;
                }
                None => break,
            }
        }
    }
}