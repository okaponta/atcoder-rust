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
        k:usize,
        wp:[(i128,i128);n],
    }
    let mut lower = 0;
    let mut upper = 10000000;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, &wp, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower as f64 / 100000.0)
}

fn is_ok(t: i128, wp: &Vec<(i128, i128)>, k: usize) -> bool {
    let mut heap = BinaryHeap::new();
    for &(w, p) in wp {
        heap.push(w * (p * 100000 - t));
    }
    let mut res = 0;
    for _i in 0..k {
        res += heap.pop().unwrap()
    }
    0 < res
}
