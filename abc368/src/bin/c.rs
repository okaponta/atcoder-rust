#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut t = 0;
    for i in 0..n {
        t += 1;
        t = f(t, h[i]);
    }
    println!("{t}");
}

fn f(t: usize, h: usize) -> usize {
    let mut lower = t - 1;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if h <= g(t, med) {
            upper = med;
        } else {
            lower = med;
        }
    }
    upper
}

fn g(a: usize, b: usize) -> usize {
    h(b) - h(a - 1)
}

fn h(a: usize) -> usize {
    let t = a / 3;
    let mut res = a - t;
    res += 3 * t;
    res
}
