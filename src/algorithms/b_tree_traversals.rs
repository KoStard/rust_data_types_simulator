use crate::data_structures::binary_tree::BinaryTree;

fn pre_order_traversal_util<T>(node: &BinaryTree<T>, vec: &mut Vec<T>) where T: Clone{
    vec.push(node.value.clone());
    if let Some(n) = &node.left {
        pre_order_traversal_util(n, vec);
    }
    if let Some(n) = &node.right {
        pre_order_traversal_util(n, vec);
    }
}

pub fn pre_order_traversal<T>(root: &BinaryTree<T>) -> Vec<T> where T: Clone{
    let mut res = Vec::new();
    pre_order_traversal_util(root, &mut res);
    res
}

pub fn demo() {
    let mut root = BinaryTree::new("F");
    let mut l = BinaryTree::new("B");
    l.set_left("A");
    l.set_right("C");
    let mut r = BinaryTree::new("G");
    r.set_left("J");
    r.set_right("I");
    root.left = Some(Box::new(l));
    root.right = Some(Box::new(r));
    println!(
        "{:?}",
        pre_order_traversal(&root)
    );
}

