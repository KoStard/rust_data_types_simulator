use std::cmp::PartialEq;
use std::fmt::Display;

pub struct MaxHeap {
    pub array: Vec<i64>,
}

impl MaxHeap {
    pub fn new() -> MaxHeap {
        MaxHeap { array: vec![] }
    }
    pub fn from_array(arr: &[i64]) -> MaxHeap {
        let mut heap = MaxHeap {
            array: arr.to_vec(),
        };
        for i in (0..=(arr.len() - 1) / 2).rev() {
            // From last parent node
            heap.max_heapify(i);
        }
        heap
    }
    // O(1)
    pub fn getMaxi(&self) -> i64 {
        self.array[0]
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        let temp = self.array[index1];
        self.array[index1] = self.array[index2];
        self.array[index2] = temp;
    }
    pub fn insert(&mut self, new_item: i64) -> &mut MaxHeap {
        let mut index = self.array.len();
        self.array.push(new_item);
        self.max_heapify_up(index);
        self
    }
    pub fn max_heapify_up(&mut self, index: usize) {
        let mut index = index;
        loop {
            let parent_index = if index == 0 { 0 } else { parent(index) };
            if index == 0 || self.array[parent_index] >= self.array[index] {
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
        self.max_heapify(0);
        res
    }
    // Will heapify subtree with root at index if it's child subtrees are already heapified
    fn max_heapify(&mut self, index: usize) {
        let left_index = 2 * index + 1;
        let right_index = left_index + 1;
        let mut max_index = index;
        if left_index < self.array.len() && self.array[left_index] > self.array[max_index] {
            max_index = left_index;
        }
        if right_index < self.array.len() && self.array[right_index] > self.array[max_index] {
            max_index = right_index
        }
        if max_index != index {
            self.swap(max_index, index);
            self.max_heapify(max_index);
        }
    }

    // It is supposed, that new_value is bigger than array[index]
    // Otherwise it can be smaller than it's child and the algorithm won't fix thats
    fn increase_key(&mut self, index: usize, new_value: i64) {
        self.array[index] = new_value;
        self.max_heapify_up(index);
    }

    // O(log n)
    pub fn delete_key(&mut self, index: usize) -> &mut MaxHeap {
        self.increase_key(index, self.getMaxi() + 1);
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

impl Display for MaxHeap {
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
    let mut heap = MaxHeap::from_array((0i64..63i64).rev().collect::<Vec<i64>>().as_slice());
    println!("{}", heap);
/*
                                                               62
                               61                                                              60
               59                              58                              57                              56
       55              54              53              52              51              50              49              48
   47      46      45      44      43      42      41      40      39      38      37      36      35      34      33      32
 31  30  29  28  27  26  25  24  23  22  21  20  19  18  17  16  15  14  13  12  11  10  9   8   7   6   5   4   3   2   1   0

*/
}
