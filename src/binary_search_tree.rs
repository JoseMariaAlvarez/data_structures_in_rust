
#[derive(Debug)]
pub struct BinarySearchTree<T: Eq + PartialOrd> {
    pub head: BSTLink<T>,
}

pub type BSTLink<T> = Option<Box<BSTNode<T>>>;


#[derive(Debug)]
pub struct BSTNode<T> {
    pub elem: T,
    pub left: BSTLink<T>,
    pub right: BSTLink<T>,
}

impl<T: Eq + PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { head: None }
    }

    fn insert_rec_aux(bl : & mut BSTLink<T>, elem : & T){
        match bl {
            Some(ref mut node) => {
                if (*elem < node.elem) {
                    Self::insert_rec_aux(& mut node.left, elem);
                } else {
                    Self::insert_rec_aux(& mut node.right, elem);
                }
            }
            None => {
                let new_node = Box::new(BSTNode {
                    elem: *elem,
                    left: None,
                    right: None,
                });
                *bl = Some(new_node);
            }
        }
    }

    pub fn insert_rec(& mut self, elem: & T) {
        let new_node = Box::new(BSTNode {
            elem: elem,
            left: None,
            right: None,
        });

        match self.head {
            Some(ref mut node) => {
                Self::insert_rec_aux(& mut node.left, elem);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }
    pub fn insert(& mut self, elem: T) {
        let new_node = Box::new(BSTNode {
            elem: elem,
            left: None,
            right: None,
        });

        match self.head {
            Some(ref mut node) => {
                if (new_node.elem < node.elem) {
                    let mut cur_link = &mut node.left;
                    while let Some(ref mut boxed_node) = cur_link {
                        cur_link = &mut boxed_node.left;
                        println!("Boxed node: {:?}", boxed_node.elem);
                    }
                    *cur_link = Some(new_node);
                } else {
                    let mut cur_link = &mut node.right;
                    while let Some(ref mut boxed_node) = cur_link {
                        cur_link = &mut boxed_node.right;
                        println!("Boxed node: {:?}", boxed_node.elem);
                    }
                    *cur_link = Some(new_node);
                }
                // let mut cur_link = &mut node.left;
                // while let Some(ref mut boxed_node) = cur_link {
                //     cur_link = &mut boxed_node.left;
                //     println!("Boxed node: {:?}", boxed_node.elem);
                // }
                // *cur_link = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    // pub fn push(&mut self, elem: T) {
    //     let new_node = Box::new(BSTNode {
    //         elem: elem,
    //         left: self.head.take(),
    //     });

    //     self.head = Some(new_node);
    // }

    pub fn pop(&mut self) -> Option<T> {
        let y = self.head.take();
        // let z = y.map(|node| {
        //     self.head = node.next;
        //     node.elem
        // });
        // z
        match y {
            Some(node) => {
                self.head = node.left;
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
            cur_link = boxed_node.left.as_ref();
            println!("{:?}", boxed_node.elem);
        }
    }

    pub fn push_last(&mut self, elem: T) 
    where T: std::fmt::Debug {
        let new_node = Box::new(BSTNode {
            elem: elem,
            left: None,
        });

        match self.head {
            Some(ref mut node) => {
                let mut cur_link = &mut node.left;
                while let Some(ref mut boxed_node) = cur_link {
                    cur_link = &mut boxed_node.left;
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

impl<T> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.left.take();
        }
    }
}

