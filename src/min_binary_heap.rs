use std::cmp::PartialEq;
use std::fmt::Display;

pub struct MinHeap {
    pub array: Vec<i64>,
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap { array: vec![] }
    }
    pub fn getMini(&self) -> i64 {
        self.array[0]
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        let temp = self.array[index1];
        self.array[index1] = self.array[index2];
        self.array[index2] = temp;
    }
    pub fn insert(&mut self, new_item: i64) -> &mut MinHeap {
        let mut index = self.array.len();
        self.array.push(new_item);
        loop {
            let parent_index = if index == 0 { 0 } else { parent(index) };
            if index == 0 || self.array[parent_index] < self.array[index] {
                break;
            } else {
                self.swap(parent_index, index);
                index = parent_index;
            }
        }
        self
    }
    pub fn extract(&mut self, item: i64) -> &mut MinHeap {
        self.array[0] = self.array[self.array.len() - 1];
        self.array.remove(self.array.len() - 1);
        let length = self.array.len();
        let mut current_level = 0;
        let mut levels = 0;
        let mut count = 0;
        let mut index = 0;
        loop {
            count += 1 << levels;
            levels += 1;
            if count >= length {
                break;
            }
        }
        loop {
            let left_child_index = left_child(index);
            let right_child_index = right_child(index);
            if current_level == levels
                || (left_child_index >= length || (self.array[index] < self.array[left_child_index]
                    && (right_child_index >= length
                        || self.array[index] < self.array[right_child_index])))
            {
                break;
            }
            if right_child_index >= index || self.array[right_child_index] >= self.array[left_child_index]{
                self.swap(index, left_child_index);
                index = left_child_index;
            } else {
                self.swap(index, right_child_index);
                index = right_child_index;
            }
            current_level += 1;
        }
        self
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn left_child(i: usize) -> usize {
    i * 2 + 1
}

fn right_child(i: usize) -> usize {
    i * 2 + 2
}
