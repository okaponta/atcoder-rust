use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        k:i64,
        a:[i64;n],
    }
    let s = ruiseki(&a);
    let mut map = HashMap::new();
    map.insert(0, 1);
    let mut sum = vec![1];
    for i in 0..n {
        let mut tmp = sum[i];
        if let Some(&v) = map.get(&(s[i + 1] - k)) {
            tmp = (MOD + tmp - v) % MOD;
        }
        *map.entry(s[i + 1]).or_insert(0) += tmp;
        *map.entry(s[i + 1]).or_insert(0) %= MOD;
        sum.push((sum[i] + tmp) % MOD);
    }
    println!("{}", (MOD + sum[n] - sum[n - 1]) % MOD);
}

fn ruiseki(a: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
