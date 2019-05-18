mod iN_struct;
mod binary_heap;
use binary_heap::Heap;

fn main() {
    let heap = Heap::from_range(0..100);
    let node = heap.root();
    println!("{}", node == node.left_child().unwrap().parent().unwrap());
    println!("{}", node.left_child().unwrap());
    println!("{}", node.right_child().unwrap());
    println!("{}", node.left_child().unwrap().left_child().unwrap());
    println!("{}", node.right_child().unwrap().right_child().unwrap());
}
