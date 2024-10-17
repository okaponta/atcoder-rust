fn main() {
    input! {
        s:Chars,
    }
    let mut map = HashMap::new();
    for i in 0..s.len() {
        map.entry(s[i]).or_insert(vec![]).push(i);
    }
    let mut ans = 0;
    for (_, v) in map {
        let n = v.len();
        let mut tmp = 0;
        let mut sum = v[0];
        for i in 1..n {
            tmp += i * (v[i] - 1) - sum;
            sum += v[i];
        }
        ans += tmp;
    }
    println!("{}", ans);
}

use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;
