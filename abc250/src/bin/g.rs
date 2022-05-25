use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[i64;n],
    }
    let mut heap = BinaryHeap::new();
    for pi in p.iter() {
        if let Some(Reverse(min)) = heap.peek() {
            if &pi < min {
                heap.push(Reverse(pi));
            } else {
                heap.pop();
                heap.push(Reverse(pi));
                heap.push(Reverse(pi));
            }
        } else {
            heap.push(Reverse(pi));
        }
    }
    let sum: i64 = p.iter().sum();
    let ans = heap.iter().fold(-sum, |s: i64, Reverse(i)| s + *i);
    println!("{}", ans);
}
