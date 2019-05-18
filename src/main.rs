mod iN_struct;
use iN_struct::{iN};

fn main() {
    println!("{}", iN::new(2) - iN::new(-2));
    println!("{}", iN::new(2) + iN::new(-2));
}
