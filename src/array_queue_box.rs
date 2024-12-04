use std::ops::Deref;

const ARRAY_SIZE: usize = 5;

#[derive(Debug, Default)]
enum Node<T> {
    #[default]
    Nil,
    Element(T),
}

#[derive(Debug)]
pub struct ArrayQueueBoxLifetime<'a, T: Default> {
    data: [Node<Box<&'a T>>; ARRAY_SIZE],
    head: usize,
    elements: usize,
}

impl<'a, T: Default> ArrayQueueBoxLifetime<'a, T> {
    pub fn new() -> Self {
        ArrayQueueBoxLifetime {
            data: std::array::from_fn(|_| Node::default()),
            // data: [Node::default(); ARRAY_SIZE],
            head: 0,
            elements: 0,
        }
    }

    pub fn enqueue(&mut self, value: &'a T) {
        // if ARRAY_SIZE <= self.elements {
        //     panic!("Queue is full");
        // }
        // if ARRAY_SIZE <= self.elements {
        //     return Error::new("Queue is full");
        // }
        if self.elements < ARRAY_SIZE {
            self.data[(self.head + self.elements) % ARRAY_SIZE] = Node::Element(Box::new(value));
            self.elements += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        if self.elements == 0 {
            return None;
        }
        let value = &self.data[self.head];
        self.head = (self.head + 1) % ARRAY_SIZE;
        self.elements -= 1;
        match value {
            Node::Nil => None,
            Node::Element(v) => Some(v.deref()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.elements == 0 {
            return None;
        }
        match &self.data[self.head] {
            Node::Nil => None,
            Node::Element(v) => Some(v.deref()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements == 0
    }

    pub fn modifyHeadElement(&mut self, value: &'a T) {
        if 0 < self.elements {
            match &mut self.data[self.head] {
                Node::Nil => (),
                Node::Element(v) => *v = Box::new(value),
            }
        }
    }
}

// impl<T: Default> Drop for ArrayQueueLifetime<T> {
//     fn drop(&mut self) {
//         println!("Dropping ArrayQueue");
//     }
// }
