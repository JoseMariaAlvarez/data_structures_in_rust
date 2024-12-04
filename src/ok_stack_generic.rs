
#[derive(Debug)]
pub struct List<T> {
    pub head: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;


#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let y = self.head.take();
        // let z = y.map(|node| {
        //     self.head = node.next;
        //     node.elem
        // });
        // z
        match y {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => {
                None
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn traverse_queue(& self) 
    where T: std::fmt::Debug {
        let mut cur_link = self.head.as_ref();
        while let Some(boxed_node) = cur_link {
            cur_link = boxed_node.next.as_ref();
            println!("{:?}", boxed_node.elem);
        }
    }

    pub fn push_last(&mut self, elem: T) 
    where T: std::fmt::Debug {
        let new_node = Box::new(Node {
            elem: elem,
            next: None,
        });

        match self.head {
            Some(ref mut node) => {
                let mut cur_link = &mut node.next;
                while let Some(ref mut boxed_node) = cur_link {
                    cur_link = &mut boxed_node.next;
                    println!("Boxed node: {:?}", boxed_node.elem);
                }
                *cur_link = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }

        //// if let Some(mut boxed_node) = self.head.as_mut() {
        ////     let mut cur_link = &mut boxed_node.next;
        ////     while let Some(mut boxed_node) = cur_link.take() {
        ////         cur_link = &mut boxed_node.next;
        ////     }
        ////     *cur_link = Some(new_node);
        //// } else {
        ////     self.head = Some(new_node);
        //// }
        //// if let Some(mut boxed_node) = self.head.as_mut() {
        ////     let mut cur_link = boxed_node.next.as_mut();
        ////     while let Some(mut boxed_node) = cur_link.as_mut() {
        ////         cur_link = boxed_node.next.as_mut();
        ////     }
        ////     *cur_link = Some(new_node);
        //// } else {
        ////     self.head = Some(new_node);
        //// }
        
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

