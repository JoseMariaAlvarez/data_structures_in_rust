use std::fmt::Debug;


#[derive(Debug)]
pub struct BinarySearchTree<T: PartialOrd + Debug> {
    pub head: BSTLink<T>,
}

pub type BSTLink<T> = Option<Box<BSTNode<T>>>;


#[derive(Debug)]
pub struct BSTNode<T> {
    pub elem: T,
    pub left: BSTLink<T>,
    pub right: BSTLink<T>,
}

impl<T: PartialOrd + Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { head: None }
    }

    fn insert_rec_aux(bl : & mut BSTLink<T>, elem : T){
        match bl {
            Some(ref mut root) => {
                if elem < root.elem {
                    Self::insert_rec_aux(& mut root.left, elem);
                } else {
                    Self::insert_rec_aux(& mut root.right, elem);
                }
            }
            None => {
                let new_node = Box::new(BSTNode {
                    elem: elem,
                    left: None,
                    right: None,
                });
                *bl = Some(new_node);
            }
        }
    }

    pub fn insert_rec(& mut self, elem: T) {
        Self::insert_rec_aux(& mut self.head, elem);
    }

    pub fn search(& self, elem: T) -> bool {
        let mut cur_link = self.head.as_ref();
        while let Some(boxed_node) = cur_link {
            if boxed_node.elem == elem {
                return true;
            } else if elem < boxed_node.elem {
                cur_link = boxed_node.left.as_ref();
            } else {
                cur_link = boxed_node.right.as_ref();
            }
        }
        false
    }

    pub fn print_aux(root: & BSTLink<T>) {
        if let Some(boxed_node) = root {
            Self::print_aux(& boxed_node.left);
            println!("{:?}", boxed_node.elem);
            Self::print_aux(& boxed_node.right);
        }
    }

    pub fn print(& self) {
        Self::print_aux(& self.head);
    }

    pub fn insert(& mut self, elem: T) {
        let new_node = Box::new(BSTNode {
            elem: elem,
            left: None,
            right: None,
        });

        match self.head {
            Some(ref mut node) => {
                let mut cur_node = &mut self.head;
                while let Some(ref mut boxed_node) = cur_node {
                    if new_node.elem < boxed_node.elem {
                        cur_node = & mut boxed_node.left;
                    } else {
                        cur_node = & mut boxed_node.right;
                    }
                }
                *cur_node = Some(new_node);
    //             if (new_node.elem < node.elem) {
    //                 let mut cur_link = &mut node.left;
    //                 while let Some(ref mut boxed_node) = cur_link {
    //                     cur_link = &mut boxed_node.left;
    //                     println!("Boxed node: {:?}", boxed_node.elem);
    //                 }
    //                 *cur_link = Some(new_node);
    //             } else {
    //                 let mut cur_link = &mut node.right;
    //                 while let Some(ref mut boxed_node) = cur_link {
    //                     cur_link = &mut boxed_node.right;
    //                     println!("Boxed node: {:?}", boxed_node.elem);
    //                 }
    //                 *cur_link = Some(new_node);
    //             }
    //             // let mut cur_link = &mut node.left;
    //             // while let Some(ref mut boxed_node) = cur_link {
    //             //     cur_link = &mut boxed_node.left;
    //             //     println!("Boxed node: {:?}", boxed_node.elem);
    //             // }
    //             // *cur_link = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

}

impl<T: PartialOrd + Debug> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.left.take();
        }
    }
}

