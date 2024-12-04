use std::{cell::RefCell, rc::Rc};
use data_structures_in_rust::bin_tree::TreeNode;
use data_structures_in_rust::bin_tree::TreeNodeRef;

pub fn tree_sum_recursive(root: Option<&TreeNodeRef>) -> i32 {
    // if `root` has `Some`thing
    // return `root.val` + left_node_val + right_node_val
    if let Some(root) = root {
        root.borrow().get_val()
                // recursively call left path
                + tree_sum_recursive(root.borrow().get_left().as_ref())
                // recursively call right path
                + tree_sum_recursive(root.borrow().get_right().as_ref())
    } else {
        // root is None (i.e. empty or null)
        // so return `0`
        0
    }
}

pub fn tree_sum(root: TreeNodeRef) -> i32 {
    let mut sum = 0i32;
    // We'll use a `vec` as a
    // stack LIFO data structure.
    // Start by adding the root node
    // to the stack.
    let mut stack = vec![root];

    while !stack.is_empty() {
        // current points to top most
        // item in the stack
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        sum += current.borrow().get_val();

        // if there is a right node,
        // then push it on top of the stack
        if let Some(right) = &current.borrow().get_right() {
            stack.push(right.to_owned());
        };
        // if there is a left node,
        // then push it on top of the stack
        if let Some(left) = &current.borrow().get_left() {
            stack.push(left.to_owned());
        };
    }
    sum
}

fn main() {
    // create a tree
    let tree = TreeNode::new_with_children(1,
        Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        Some(Rc::new(RefCell::new(TreeNode::new(3)))));
    let sum = tree_sum(Rc::new(RefCell::new(tree)));
    println!("Sum of tree nodes: {}", sum);
}