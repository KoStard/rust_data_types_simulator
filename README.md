# rust_data_types_simulator
Simulating data types in rust to learn how they work.
- Added signed integers with variable bites count, e.g. i7, i59... - Can see their bites to understand negative numbers.
- MinHeaps - getMini, insert, extract, delete_key - implemented min_heapify and min_heapify_up, implemented Display for MinHeap struct, so this is an output example for MinHeap from range (0..63).rev().
```
                                                               0                                                                
                               16                                                              1                                
               24                              17                              8                               2                
       28              25              20              18              12              9               4               3        
   30      29      26      44      22      21      19      40      14      13      10      36      6       5       33      32   
 31  47  55  46  27  45  54  59  23  43  53  42  61  41  52  58  15  39  51  38  11  37  50  57  7   35  49  34  60  62  48  56 
 ```
 - MaxHeaps...
 ```
                                                                62                                                               
                               61                                                              60                               
               59                              58                              57                              56               
       55              54              53              52              51              50              49              48       
   47      46      45      44      43      42      41      40      39      38      37      36      35      34      33      32   
 31  30  29  28  27  26  25  24  23  22  21  20  19  18  17  16  15  14  13  12  11  10  9   8   7   6   5   4   3   2   1   0  
 ```
## Implementing some algorithms
- k_largest_elements_in_array - with sorting, buffers, minHeaps, maxHeaps
