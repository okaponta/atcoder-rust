use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        ab:[(usize,usize);n],
    }
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((ab[i].1, i, 0));
    }
    let mut ans = 0;
    for _ in 0..k {
        let (v, i, j) = heap.pop().unwrap();
        ans += v;
        if j == 0 {
            heap.push((ab[i].0 - ab[i].1, i, 1));
        }
    }
    println!("{}", ans);
}
