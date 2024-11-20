use std::{cmp::Reverse, f64::consts::PI};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut r:[i64;n],
    }
    r.sort_by_key(|&i| Reverse(i));
    let ans = (0..n)
        .into_iter()
        .map(|i| (-1i64).pow(i as u32) * r[i] * r[i])
        .sum::<i64>() as f64
        * PI;
    println!("{}", ans);
}
