use std::fmt;
use std::i8;

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut num: i64 = self
            .bits
            .iter()
            .enumerate()
            .map(|(index, b)| {
                if index == 0 {return 0;}
                if *b {
                    2i64.pow((self.bits.len() - index - 1) as u32)
                } else {
                    0i64
                }
            })
            .sum();
        if self.bits[0] {
            num-=2i64.pow(N as u32-1);
        }
        write!(f, "{}", num)
    }
}
impl fmt::Binary for iN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let num: String = self
            .bits
            .iter()
            .map(|b| {
                if *b {'1'} else {'0'}
            })
            .collect::<String>();
        write!(f, "{}", num)
    }
}


// Controlling bits count from here
const N: usize = 8;

fn main() {
    for i in i8::MIN as i64 - 100..i8::MAX as i64 + 100 {
        let n = iN::new(i);
        println!("{} -> {} = {} = {:b}", i, i as i8, n, n);
    }
}
