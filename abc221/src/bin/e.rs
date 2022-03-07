use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut pp = vec![1; n + 1];
    for i in 1..=n {
        pp[i] = pp[i - 1] * 2 % MOD;
    }
    let ca = compress(&a);
    let mut fw = FenwickTree::new(ca.len());
    let mut ans = 0;
    for i in 0..n {
        let index = *ca.get(&a[i]).unwrap();
        if i >= 1 {
            ans += fw.sum(index + 1) % MOD * pp[i - 1] % MOD;
            ans %= MOD;
        }
        fw.add(index, mod_pow(pp[i], MOD - 2))
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

struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(len: usize) -> Self {
        Self {
            len,
            data: vec![0; len],
        }
    }

    fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while i as usize <= self.len {
            self.data[(i - 1) as usize] += v;
            i += i & -i;
        }
    }

    fn sum(&self, len: usize) -> i64 {
        assert!(len <= self.len);
        let mut len = len as i32;
        let mut sum = 0i64;
        while len > 0 {
            sum += self.data[(len - 1) as usize];
            len -= len & -len;
        }
        sum
    }
}

fn mod_pow(r: i64, n: i64) -> i64 {
    let mut k = n;
    let mut s = r;
    let mut t = 1;
    while k > 0 {
        if k & 1 == 1 {
            t = (t * s) % MOD;
        }
        s = (s * s) % MOD;
        k >>= 1;
    }
    t
}
