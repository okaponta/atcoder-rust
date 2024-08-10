use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        d:i64,
        mut xy:[(i64,i64);n],
    }
    let mut x = vec![];
    let mut y = vec![];
    for &(xi, yi) in &xy {
        x.push(xi);
        y.push(yi);
    }
    let xv = f(x, d, n as i64);
    let mut yv = f(y, d, n as i64);
    yv.sort();
    let mut ans = 0;
    for xx in xv {
        ans += yv.upper_bound(&(d - xx));
    }
    println!("{}", ans);
}

fn f(x: Vec<i64>, d: i64, n: i64) -> Vec<i64> {
    let mut tmp = 0;
    let mut map = HashMap::new();
    for &xi in &x {
        tmp += (-2_000_010 - xi).abs();
        *map.entry(xi).or_insert(0) += 1;
    }
    let mut plus = 0;
    let mut minus = n;
    let mut v = vec![];
    for i in -2_000_010..2_000_010 {
        if tmp <= d {
            v.push(tmp);
        }
        let diff = map.get(&i).unwrap_or(&0);
        plus += diff;
        minus -= diff;
        tmp += plus - minus;
    }
    v
}
