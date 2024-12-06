use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn cons(elem: T, tail: & List<T>) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: tail.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        match self.head {
            Some(ref node) => List {
                head: node.next.clone(),
            },
            None => List { head: None },
        }
    }
    
    pub fn head(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.elem),
            None => None,
        }
    }

//     pub fn tail(&self) -> List<T> {
// //  List { head: self.head.and_then(|node| node.next.clone()) }
// // cannot move out of `self.head` which is behind a shared reference
// // move occurs because `self.head` has type `Option<Rc<cons_list::Node<T>>>`, 
// // which does not implement the `Copy` traitrustc
// // `self.head` moved due to this method call
// // `Option::<T>::and_then` takes ownership of the receiver `self`, 
// // which moves `self.head`
// // help: consider calling `.as_ref()` or `.as_mut()`
// // self.head: Option<Rc<Node<T>>>
// // self.head.as_ref(): Option<&Rc<Node<T>>>
// // to borrow the type's contents
// // you can `clone` the value and consume it, 
// // but this might not be your desired behavior
// // consider cloning the value if the performance cost is acceptable: `.clone()`
// // List { head: self.head.clone().and_then(|node| node.next.clone()) }

//         List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
//     }

// pub fn head(&self) -> Option<&T> {
//         self.head.as_ref().map(|node| &node.elem)
//     }
}
