use std::collections::BinaryHeap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        w:usize,
        wv:[(usize,usize);n],
    }
    let mut dp = vec![0; w + 1];
    let mut e = vec![vec![]; 3001];
    for (w, v) in wv {
        e[w].push(v);
    }
    for i in 0..=3000 {
        if e[i].len() == 0 {
            continue;
        }
        let mut heap = BinaryHeap::new();
        for &v in &e[i] {
            heap.push(v - 1);
        }
        let mut vv = vec![0];
        while let Some(v) = heap.pop() {
            if 6000 / i < vv.len() {
                break;
            }
            vv.push(v);
            if 2 < v {
                heap.push(v - 2);
            }
        }
        for j in (0..w).rev() {
            let mut k = 1;
            while j + k * i <= w && k < vv.len() {
                dp[j + k * i] = dp[j + k * i].max(dp[j + (k - 1) * i] + vv[k]);
                k += 1;
            }
        }
    }
    println!("{}", dp[w]);
}
