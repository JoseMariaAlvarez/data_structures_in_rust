use data_structures_in_rust::binary_search_tree::BinarySearchTree;

fn main(){
    let mut bst = BinarySearchTree::new();
    bst.insert_rec(3);
    bst.insert_rec(2);
    bst.insert_rec(1);
    bst.insert_rec(4);
    bst.insert_rec(5);

    bst.print();

    let mut bst2 = BinarySearchTree::new();
    bst2.insert(3);
    bst2.insert(2);
    bst2.insert(1);
    bst2.insert(4);
    bst2.insert(5);

    bst2.print();
}