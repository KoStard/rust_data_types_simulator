use std::fmt::Display;
use std::cmp::PartialEq;

pub struct Heap {
    array: Vec<i64>,
}

pub struct Node<'node_lifetime> {
    heap: &'node_lifetime Heap,
    row: u32,
    col: u32,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { array: vec![] }
    }
    pub fn from_vec(v: &Vec<i64>) -> Heap {
        Heap { array: v.clone() }
    }
    pub fn from_range(r: std::ops::Range<i64>) -> Heap{
        Heap { array: r.collect::<Vec<i64>>()}
    }
    pub fn get_node_at(&self, row: u32, col: u32) -> Node {
        Node {
            heap: self,
            row: row,
            col: col,
        }
    }
    pub fn root(&self) -> Node{
        Node {
            heap: self,
            row: 0,
            col: 0
        }
    }
}

impl<'node_lifetime> Node<'node_lifetime> {
    pub fn index(&self) -> usize {
        (2u32.pow(self.row) - 1 + self.col) as usize
    }
    pub fn value(&self) -> i64 {
        self.heap.array[self.index()]
    }
    pub fn parent(&self) -> Option<Node> {
        if self.row == 0 {
            None
        } else {
            Some(Node {
                heap: self.heap,
                row: self.row - 1,
                col: self.col / 2,
            })
        }
    }
    pub fn left_child(&self) -> Option<Node> {
        if self.index() * 2 + 1 > self.heap.array.len() {
            None
        } else {
            Some(Node {
                heap: self.heap,
                row: self.row + 1,
                col: self.col * 2,
            })
        }
    }
    pub fn right_child(&self) -> Option<Node> {
        if self.index() * 2 + 2 > self.heap.array.len() {
            None
        } else {
            Some(Node {
                heap: self.heap,
                row: self.row + 1,
                col: self.col * 2 + 1,
            })
        }
    }
}


impl<'node_lifetime> Display for Node<'node_lifetime> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Node (row: {}, col: {}) -> {}", self.row, self.col, self.value())
    }
}


impl<'node_lifetime> PartialEq for Node<'node_lifetime> {
    //- Comparing only values
    fn eq(&self, other: &Node) -> bool{
        self.value() == other.value()
    }
}
