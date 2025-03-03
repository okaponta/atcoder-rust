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
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(i);
    }
    let mut ans = 1 << 60;
    for (_, v) in map {
        for vv in v.windows(2) {
            ans = ans.min(vv[1] - vv[0]);
        }
    }
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", ans + 1);
    }
}
