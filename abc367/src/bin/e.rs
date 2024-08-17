#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut k:usize,
        mut x:[Usize1;n],
        mut a:[usize;n],
    }
    while 0 < k {
        if k & 1 == 1 {
            let mut b = vec![];
            for i in 0..n {
                b.push(a[x[i]]);
            }
            a = b;
        }
        x = next(n, x);
        k >>= 1;
    }
    println!("{}", a.iter().join(" "));
}

fn next(n: usize, x: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..n {
        res.push(x[x[i]]);
    }
    res
}
