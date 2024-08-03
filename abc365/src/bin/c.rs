#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, m, n, &a) {
            lower = med;
        } else {
            upper = med;
        }
    }
    if upper == 1 << 60 {
        println!("infinite");
        return;
    }
    println!("{}", lower);
}

fn is_ok(med: usize, m: usize, n: usize, a: &Vec<usize>) -> bool {
    let mut tmp = 0;
    for i in 0..n {
        tmp += a[i].min(med);
    }
    tmp <= m
}
