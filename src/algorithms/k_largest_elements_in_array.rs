// mod min_binary_heap;
// use min_binary_heap;

use crate::max_binary_heap::MaxHeap;
use crate::min_binary_heap::MinHeap;

pub enum Modes {
    Sorting,
    Buffers,
    BigMaxMeap,
    SmallMinMeap,
}

pub fn k_largest_elements_in_array(array: &Vec<i64>, k: usize, mode: Modes) -> Vec<i64> {
    //- With sorting
    match mode {
        Modes::Sorting => {
            // O(n log n)
            let mut array = array.clone();
            array.sort();
            return array[std::cmp::max(array.len() - std::cmp::min(k, array.len()), 0)..]
                .iter()
                .map(|e| *e)
                .collect();
        }
        Modes::Buffers => {
            // O (n * k)
            let mut res = vec![];
            for &n in array.iter() {
                if res.len() < k {
                    res.push(n);
                } else {
                    let mut current = n;
                    for (index, &e) in res.clone().iter().enumerate().rev() {
                        if current > e {
                            res[index] = current;
                            current = e;
                        }
                    }
                }
            }
            res
        }
        // Order will be lost - is reversely sorted
        // Faster than SmallMinMeap when n is big and k is small
        Modes::BigMaxMeap => {
            // O(n + k log n) - O(n log n) is the worst case - basically is a sorting algorithm
            let mut heap = MaxHeap::from_array(&array); // O(n)
            let mut res = vec![];
            for _ in 0..std::cmp::min(k, array.len()) {
                // O(k log n)
                res.push(heap.extract()) // O(log n)
            }
            res
        }
        // Unordered - linear when n == k
        // Faster than BigMaxMeap when k is big
        Modes::SmallMinMeap => {
            // O(k + (n-k) log k)
            let mut heap = MinHeap::from_array(&array[..std::cmp::min(k, array.len())]); // O(k)
            if k < array.len() {
                for &e in &array[k..] {
                    // O((n-k) log k)
                    if e > heap.getMini() {
                        heap.change_root_value(e); // O(log k)
                    }
                }
            }
            heap.as_vec() // O(k)
        }
    }
}

pub fn demo() {
    println!(
        "{:?}",
        k_largest_elements_in_array(&(0..100000).collect::<Vec<i64>>(), 10, Modes::SmallMinMeap)
    );
}
