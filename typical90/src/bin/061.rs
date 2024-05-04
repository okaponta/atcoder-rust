use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q:usize,
        tx:[(u8,usize);q],
    }
    let mut q = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => q.push_front(x),
            2 => q.push_back(x),
            _ => println!("{}", q[x - 1]),
        }
    }
}
