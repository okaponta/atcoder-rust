#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut lower = 0;
    let mut upper = (n / 2) + 1;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(n, med, &a) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(n: usize, k: usize, a: &Vec<usize>) -> bool {
    (0..k).all(|i| a[i] * 2 <= a[n - k + i])
}
