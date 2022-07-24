use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        c:[Usize1;n],
        x:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        (*map.entry(c[i]).or_insert(vec![])).push(x[i]);
    }
    let mut ans = inversion_num(x);
    for v in map.values_mut() {
        let map = compress(v);
        let compressed = v.iter().map(|x| map[x] + 1).collect_vec();
        ans -= inversion_num(compressed);
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

fn inversion_num(a: Vec<usize>) -> i64 {
    let max = *a.iter().max().unwrap();
    let mut inv = 0;
    let mut fw = FenwickTree::new(max);
    for i in 0..a.len() {
        inv += i as i64 - fw.sum(a[i]);
        fw.add(a[i], 1);
    }
    inv
}

struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }
}
