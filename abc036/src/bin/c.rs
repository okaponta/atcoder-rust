use std::collections::{HashMap, HashSet};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let map = compress(&a);
    let ans = (0..n).into_iter().map(|i| map[&a[i]]).collect_vec();
    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}
