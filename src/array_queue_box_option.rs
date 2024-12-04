const ARRAY_SIZE: usize = 5;

#[derive(Debug)]
pub struct ArrayQueueBoxOption<T: Default> {
    data: [Option<Box<T>>; ARRAY_SIZE],
    head: usize,
    elements: usize,
}

impl<T: Default> ArrayQueueBoxOption<T> {
    pub fn new() -> Self {
        ArrayQueueBoxOption {
            // data: std::array::from_fn(|_| const { None }),
            data: [const { None } ; ARRAY_SIZE],
            head: 0,
            elements: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        // if ARRAY_SIZE <= self.elements {
        //     panic!("Queue is full");
        // }
        // if ARRAY_SIZE <= self.elements {
        //     return Error::new("Queue is full");
        // }
        if self.elements < ARRAY_SIZE {
            self.data[(self.head + self.elements) % ARRAY_SIZE] = Some(Box::new(value));
            self.elements += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        if self.elements == 0 {
            return None;
        }
        let value = self.data[self.head].as_ref();
        self.head = (self.head + 1) % ARRAY_SIZE;
        self.elements -= 1;
        value.map(|node| node.as_ref())
    }

    pub fn peek(&self) -> Option<&T> {
        if self.elements == 0 {
            return None;
        }
        self.data[self.head].as_ref().map(|node| node.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.elements == 0
    }
}

impl<T: Default> Drop for ArrayQueueBoxOption<T> {
    fn drop(&mut self) {
        println!("Dropping ArrayQueue");
    }
}

