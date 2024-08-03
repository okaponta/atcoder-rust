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
    let mut v = vec![];
    for i in 0..n {
        v.push((a[i], i + 1));
    }
    v.sort();
    v.reverse();
    println!("{}", v[1].1);
}
