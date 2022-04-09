use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let amap = compress(&a);
    let len = amap.len();
    let mut idx = vec![0; len];
    for i in 0..k {
        idx[amap[&a[i]]] = idx[amap[&a[i]]].max(i + 1);
    }
    let mut mx = 0;
    for i in 0..len {
        idx[i] = idx[i].max(mx);
        mx = mx.max(idx[i]);
    }
    idx.insert(0, 0);
    let mut ans = 1 << 60;
    for i in k..n {
        let tmp = idx[amap[&a[i]]];
        if tmp == 0 {
            continue;
        }
        ans = ans.min(i + 1 - tmp);
    }
    if ans == 1 << 60 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}

fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}
