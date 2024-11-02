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
    let mut ans = vec![];
    for i in 0..n {
        if let Some(&j) = map.get(&a[i]) {
            ans.push(j as i64);
        } else {
            ans.push(-1);
        }
        map.insert(a[i], i + 1);
    }
    println!("{}", ans.iter().join(" "));
}
