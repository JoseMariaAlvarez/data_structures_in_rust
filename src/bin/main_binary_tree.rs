use data_structures_in_rust::binary_tree::BinaryTree;

pub fn tree_sum_recursive(bt: &BinaryTree<i32>) -> i32 {
    // if `root` has `Some`thing
    // return `root.val` + left_node_val + right_node_val
    if let Some(root) = bt.root() {
        *root   // root value
                // recursively call left path
                + tree_sum_recursive(&bt.left())
                // recursively call right path
                + tree_sum_recursive(&bt.right())
    } else {
        // root is None (i.e. empty or null)
        // so return `0`
        0
    }
}

pub fn tree_sum(bt: &BinaryTree<i32>) -> i32 {
    let mut sum = 0i32;
    // We'll use a `vec` as a
    // stack LIFO data structure.
    // Start by adding the root node
    // to the stack.
    let mut stack = vec![bt.clone()];

    while !stack.is_empty() {
        // current points to top most
        // item in the stack
        let current = stack.pop().unwrap();
        sum += current.root().unwrap_or(&0);

        // if there is a right node,
        // then push it on top of the stack
        if let Some(_) = bt.right().root() {
            stack.push(bt.right().clone());
        };
        // if there is a left node,
        // then push it on top of the stack
        if let Some(_) = bt.left().root() {
            stack.push(bt.right().clone());
        };
    }
    sum
}

fn main() {
    // create a tree
    let btree = BinaryTree::cons(1,
        &BinaryTree::cons(2, &BinaryTree::new(), &BinaryTree::new()),
        &BinaryTree::cons(3, &BinaryTree::new(), &BinaryTree::new()));
    let sum = tree_sum_recursive(&btree);
    println!("Sum of tree nodes: {}", sum);
    let sum = tree_sum_recursive(&btree);
    println!("Sum of tree nodes: {}", sum);
}