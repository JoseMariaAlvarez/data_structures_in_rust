// https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_with_children(
        val: i32,
        left: Option<TreeNodeRef>,
        right: Option<TreeNodeRef>,
    ) -> Self {
        TreeNode { val, left, right }
    }

    pub fn set_left(&mut self, left: TreeNodeRef) {
        self.left = Some(left);
    }
    pub fn set_right(&mut self, right: TreeNodeRef) {
        self.right = Some(right);
    }
    pub fn get_val(&self) -> i32 {
        self.val
    }
    pub fn get_left(&self) -> Option<TreeNodeRef> {
        self.left.clone()
    }
    pub fn get_right(&self) -> Option<TreeNodeRef> {
        self.right.clone()
    }
  }
pub type TreeNodeRef = Rc<RefCell<TreeNode>>;