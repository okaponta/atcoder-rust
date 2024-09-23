use std::collections::BTreeSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let ih = h
        .iter()
        .enumerate()
        .sorted_by_key(|k| k.1)
        .rev()
        .collect::<Vec<_>>();
    let mut diff = vec![0; n + 1];
    let mut set = BTreeSet::new();
    for (i, _) in ih {
        diff[*set.range(..i).last().unwrap_or(&0)] += 1;
        diff[i] -= 1;
        set.insert(i);
    }
    let mut ans = ruiseki(&diff);
    ans.remove(0);
    ans.pop();
    println!("{}", ans.iter().join(" "));
}

fn ruiseki(a: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
