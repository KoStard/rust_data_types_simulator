use std::fmt;
use std::i8;
use std::ops::{Add, Sub};

fn vec_to_iN(v: Vec<bool>) -> iN {
    let mut res = [false; N];
    for i in 0..N {
        res[i] = v[i];
    }
    iN { bits: res }
}

struct iN {
    bits: [bool; N],
}

impl iN {
    fn new(value: i64) -> iN {
        let max = 2i64.pow((N - 1) as u32) - 1;
        let min = -2i64.pow((N - 1) as u32);
        let d = max - min + 1;
        let mut value_in_range = value % d;
        value_in_range = value_in_range
            + if value_in_range > max {
                -d
            } else if value_in_range < min {
                d
            } else {
                0
            };
        let mut res = [false; N];
        // let f = format!("{value:0N$b}", value=value_in_range, N=N);
        let f = format!("{value:0N$b}", value = value_in_range, N = N)
            .chars()
            .rev()
            .collect::<Vec<char>>()[..N]
            .iter()
            .rev()
            .map(|c| *c == '1')
            .collect::<Vec<bool>>();
        for i in 0..N {
            res[i] = f[i];
        }
        iN { bits: res }
    }
}
impl fmt::Display for iN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut num: i64 = self
            .bits
            .iter()
            .enumerate()
            .map(|(index, b)| {
                if index == 0 {
                    return 0;
                }
                if *b {
                    2i64.pow((self.bits.len() - index - 1) as u32)
                } else {
                    0i64
                }
            })
            .sum();
        if self.bits[0] {
            num -= 2i64.pow(N as u32 - 1);
        }
        write!(f, "{}", num)
    }
}
impl fmt::Binary for iN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num: String = self
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        write!(f, "{}", num)
    }
}

impl Add for iN {
    type Output = iN;
    fn add(self, other: iN) -> iN {
        let mut res = [false; N];
        let mut held_value = false;
        // println!("{:?}", self.bits);
        // println!("{:?}", other.bits);
        if other.bits[0] {
            return self
                - vec_to_iN(other.bits.iter().map(|b| !*b).collect::<Vec<bool>>())
                - iN::new(1);
        }
        for (index, b) in self.bits.iter().enumerate().rev() {
            if *b && other.bits[index] {
                res[index] = held_value;
                held_value = true;
            } else if *b || other.bits[index] {
                res[index] = true;
                if held_value {
                    res[index] = false;
                }
            } else {
                res[index] = held_value;
                held_value = false;
            }
        }
        if held_value {
            panic!("Addition with overflow");
        }
        // println!("{:?}", res);
        iN { bits: res }
    }
}

impl Sub for iN {
    type Output = iN;
    fn sub(self, other: iN) -> iN {
        let mut res = [false; N];
        let mut held_value = false;
        // println!("{:?}", self.bits);
        // println!("{:?}", other.bits);
        if other.bits[0] {
            return self
                + vec_to_iN(other.bits.iter().map(|b| !*b).collect::<Vec<bool>>())
                + iN::new(1);
        }
        // println!("{:?}", other.bits);
        for (index, b) in self.bits.iter().enumerate().rev() {
            if *b && other.bits[index] {
                res[index] = false;
                if held_value {
                    res[index] = !res[index];
                }
            } else if !*b && other.bits[index] {
                res[index] = true;
                held_value = true;
            } else if *b && !other.bits[index] {
                res[index] = true;
                if held_value {
                    res[index] = !res[index];
                    held_value = false;
                }
            } else {
                res[index] = false;
                if held_value {
                    res[index] = !res[index];
                }
            }
        }
        if held_value {
            panic!("Substraction with overflow");
        }
        // println!("{:?}", res);
        iN { bits: res }
    }
}

// Controlling bits count from here
// Add +, -, *, / ability
// Allow converting to other types
// Check use std::ops; for that
// Maybe use vectors to allow create multiple integers with different numbering system
const N: usize = 4;

fn main() {
    println!("{}", iN::new(2) - iN::new(-2));
    println!("{}", iN::new(2) + iN::new(-2));
}
