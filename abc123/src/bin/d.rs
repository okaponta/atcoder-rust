use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:usize,
        y:usize,
        z:usize,
        k:usize,
        a:[i64;x],
        b:[i64;y],
        c:[i64;z],
    }
    let mut heap = BinaryHeap::new();
    for i in 0..x {
        for j in 0..y {
            heap.push(-(a[i] + b[j]));
            if k < heap.len() {
                heap.pop();
            }
        }
    }
    let d = heap.into_iter().collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    for i in 0..z {
        for j in 0..d.len() {
            heap.push(-(c[i] - d[j]));
            if k < heap.len() {
                heap.pop();
            }
        }
    }
    let mut ans = heap.into_iter().rev().map(|i| -i).collect::<Vec<_>>();
    ans.sort_by(|a, b| b.cmp(a));
    for ans in ans {
        println!("{}", ans);
    }
}
