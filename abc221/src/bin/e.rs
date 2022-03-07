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
        let rank = *ca.get(&a[i]).unwrap() + 1;
        if i >= 1 {
            ans += fw.query(rank) % MOD * pp[i - 1] % MOD;
            ans %= MOD;
        }
        fw.update(rank, mod_pow(pp[i], MOD - 2))
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
    v: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { v: vec![0; n + 1] }
    }

    fn update(&mut self, x: usize, val: i64) {
        let mut x = x as i64;
        while x < self.v.len() as i64 {
            self.v[x as usize] += val;
            x += x & -x;
        }
    }

    fn query(&mut self, x: usize) -> i64 {
        let mut x = x as i64;
        let mut ans = 0;
        while x > 0 {
            ans += self.v[x as usize];
            x -= x & -x;
        }
        ans
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
