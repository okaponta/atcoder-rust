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
    }
    let mut a = vec![];
    let mut c = vec![];
    for _ in 0..n {
        input! {k:usize, ai:[usize;k]}
        c.push(k);
        let mut map = HashMap::new();
        for i in 0..k {
            *map.entry(ai[i]).or_insert(0usize) += 1;
        }
        a.push(map);
    }
    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let mut count = 0;
            for (k, v) in a[i].iter() {
                if a[j].contains_key(k) {
                    count += v * a[j][k];
                }
            }
            let tmp = count as f64 / (c[i] * c[j]) as f64;
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}
