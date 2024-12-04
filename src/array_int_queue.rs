const ARRAY_SIZE: usize = 5;

#[derive(Debug)]
pub struct ArrayIntQueue {
    data: [i32; ARRAY_SIZE],
    head: usize,
    elements: usize,
}

impl ArrayIntQueue {
    pub fn new() -> Self {
        ArrayIntQueue {
            data: [0; ARRAY_SIZE],
            head: 0,
            elements: 0,
        }
    }

    pub fn enqueue(&mut self, value: i32) {
        // if self.elements == ARRAY_SIZE {
        //     panic!("Queue is full");
        // }
        // if self.elements == ARRAY_SIZE {
        //     return Error::new("Queue is full");
        // }
        if self.elements < ARRAY_SIZE {
            self.data[(self.head + self.elements) % ARRAY_SIZE] = value;
            self.elements += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.elements == 0 {
            return None;
        }
        let value = self.data[self.head];
        self.head = (self.head + 1) % ARRAY_SIZE;
        self.elements -= 1;
        Some(value)
    }

    pub fn peek(&self) -> Option<i32> {
        if self.elements == 0 {
            return None;
        }
        Some(self.data[self.head])
    }

    pub fn is_empty(&self) -> bool {
        self.elements == 0
    }
    pub fn is_full(&self) -> bool {
        self.elements == ARRAY_SIZE
    }
}

impl Drop for ArrayIntQueue {
    fn drop(&mut self) {
        println!("Dropping ArrayIntQueue");
    }
}

