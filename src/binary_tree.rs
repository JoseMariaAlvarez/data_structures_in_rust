use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct BinaryTree<T> {
    root: BTLink<T>,
}

type BTLink<T> = Option<Rc<BTNode<T>>>;

#[derive(Clone, Debug)]
struct BTNode<T> {
    elem: T,
    left: BTLink<T>,
    right: BTLink<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn cons(elem: T, 
                left: & BinaryTree<T>,
                right: & BinaryTree<T>) -> BinaryTree<T> {
        BinaryTree {
            root: Some(Rc::new(BTNode {
                elem: elem,
                left: left.root.clone(),
                right: right.root.clone(),
            })),
        }
    }
    
    pub fn root(&self) -> Option<&T> {
        match self.root {
            Some(ref node) => Some(&node.elem),
            None => None,
        }
    }

    pub fn left(&self) -> BinaryTree<T> {
        match self.root {
            Some(ref node) => BinaryTree {
                root: node.left.clone(),
            },
            None => BinaryTree { root: None },
        }
    }

    pub fn right(&self) -> BinaryTree<T> {
        match self.root {
            Some(ref node) => BinaryTree {
                root: node.right.clone(),
            },
            None => BinaryTree { root: None },
        }
    }
}
