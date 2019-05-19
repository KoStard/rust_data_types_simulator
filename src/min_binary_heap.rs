use std::cmp::PartialEq;
use std::fmt::Display;

pub struct MinHeap {
    pub array: Vec<i64>,
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap { array: vec![] }
    }
    pub fn from_array(arr: &[i64]) -> MinHeap {
        let mut heap = MinHeap {
            array: arr.to_vec(),
        };
        for i in (0..=(arr.len() - 1) / 2).rev() {
            // From last parent node
            heap.min_heapify(i);
        }
        heap
    }
    // O(1)
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
        self.min_heapify_up(index);
        self
    }
    pub fn min_heapify_up(&mut self, index: usize) {
        let mut index = index;
        loop {
            let parent_index = if index == 0 { 0 } else { parent(index) };
            if index == 0 || self.array[parent_index] < self.array[index] {
                break;
            } else {
                self.swap(parent_index, index);
                index = parent_index;
            }
        }
    }
    pub fn levels(&self) -> u32 {
        let length = self.array.len();
        let mut count = 0;
        let mut levels = 0;
        loop {
            count += 1 << levels;
            levels += 1;
            if count >= length {
                break;
            }
        }
        levels
    }
    // O(log n)
    pub fn extract(&mut self) -> i64 {
        let res = self.array[0];
        self.array[0] = self.array[self.array.len() - 1];
        self.array.remove(self.array.len() - 1);
        self.min_heapify(0);
        res
    }
    // Will heapify subtree with root at index if it's child subtrees are already heapified
    fn min_heapify(&mut self, index: usize) {
        let left_index = 2 * index + 1;
        let right_index = left_index + 1;
        let mut min_index = index;
        if left_index < self.array.len() && self.array[left_index] < self.array[min_index] {
            min_index = left_index;
        }
        if right_index < self.array.len() && self.array[right_index] < self.array[min_index] {
            min_index = right_index
        }
        if min_index != index {
            self.swap(min_index, index);
            self.min_heapify(min_index);
        }
    }

    // It is supposed, that new_value is smaller than array[index]
    // Otherwise it can be bigger than it's child and the algorithm won't fix thats
    fn decrease_key(&mut self, index: usize, new_value: i64) {
        self.array[index] = new_value;
        self.min_heapify_up(index);
    }

    // O(log n)
    pub fn delete_key(&mut self, index: usize) -> &mut MinHeap {
        self.decrease_key(index, self.getMini() - 1);
        self.extract();
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

impl Display for MinHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let levels = self.levels();
        let mut last = 0;
        for i in 0..levels {
            write!(
                f,
                "{}",
                (last..std::cmp::min(last + (1 << i), self.array.len()))
                    .map(|index| format!(
                        "{:^w$}",
                        self.array[index],
                        w = (((2<<levels) / (1 << i)) as usize)
                    ))
                    .collect::<String>()
            )?;
            if i < levels - 1 {
                write!(f, "\n")?;
            }
            last += 1 << i;
        }
        write!(f, "")
    }
}


pub fn demo() {
    let mut heap = MinHeap::from_array((0i64..50i64).rev().collect::<Vec<i64>>().as_slice());
    println!("{}", heap);
/*
                                                                0
                               3                                                               1
               11                              4                               2                               19
       15              12              7               5               25              23              21              20
   17      16      13      31      9       8       6       27      26      38      24      37      22      36      43      35
 18  34  42  33  14  32  41  46  10  30  40  29  48  28  39  45  44  47  49
*/

    // let mut heap = MinHeap::from_array(&[10,9,8,7,6,5,4,3,2,1]);
    // println!("{:?}", heap.array);
    // heap.insert(-5);
    // heap.delete_key(0);
    // println!("{:?}", heap.array);
    // println!("{}", heap);
    // while heap.array.len() != 0 {
    //     println!("{}", heap.extract());
    // }
}
