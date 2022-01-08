use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
       n:usize,k:usize,
       p:[i32;n],
    }
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        heap.push(-p[i]);
    }
    println!("{}", -heap.peek().unwrap());
    for i in k..n {
        heap.push(-p[i]);
        heap.pop();
        println!("{}", -heap.peek().unwrap());
    }
}
