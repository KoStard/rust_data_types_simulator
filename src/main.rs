mod iN_struct;
mod min_binary_heap;
use min_binary_heap::MinHeap;

fn main() {
    let mut heap = MinHeap::new();
    heap.insert(15)
        .insert(25)
        .insert(35)
        .extract(25)
        .insert(4)
        .extract(4);
    println!("{:?}", heap.array);
}
