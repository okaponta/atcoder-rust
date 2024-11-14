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
    let mut ans = 1 << 60;
    for i in 0..=100 {
        ans = ans.min(solve(i, n, &a));
    }
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

fn solve(k: usize, n: usize, a: &Vec<usize>) -> usize {
    let mut tmp = 0;
    let mut count = 0;
    let mut res = 0;
    for i in 0..n {
        count += 1;
        tmp += a[i];
        if tmp == k * count {
            tmp = 0;
            count = 0;
        } else {
            res += 1;
        }
    }
    if count != 0 {
        1 << 60
    } else {
        res
    }
}
