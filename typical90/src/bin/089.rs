use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        k:i64,
        a:[usize;n],
    }
    let mut cl = vec![0; n];
    let mut left = n;
    let com = compress(&a);
    let max = com.len();
    let mut inv = 0;
    let mut fw = FenwickTree::new(max);
    for i in (0..n).rev() {
        while 0 < left && inv + fw.sum(com[&a[left - 1]] - 1) <= k {
            left -= 1;
            inv += fw.sum(com[&a[left]] - 1);
            fw.add(com[&a[left]], 1);
        }
        cl[i] = left;
        inv -= fw.sum(com.len()) - fw.sum(com[&a[i]]);
        fw.add(com[&a[i]], -1);
    }
    let mut dp = vec![0; n + 1];
    let mut dp2 = vec![0; n + 2];
    dp[0] = 1;
    dp2[1] = 1;
    for i in 0..n {
        dp[i + 1] = (MOD + dp2[i + 1] - dp2[cl[i]]) % MOD;
        dp2[i + 2] = (dp2[i + 1] + dp[i + 1]) % MOD;
    }
    println!("{}", dp[n]);
}

// 座標圧縮
// Mapを返却する。戻り値のmap[元の値]が圧縮された値になる
fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i + 1);
    }
    result
}

// フェニック木。以下2つができる。1-indexedなので注意
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // aiをvで置き換える
    pub fn update(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let cur = self.range(i, i);
        self.add(i, v - cur);
    }

    // a1+a2+...aiを計算する
    pub fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }

    // ai+...+ajを計算する
    pub fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }

    // 和がs以下の位置を返却
    pub fn lower(&self, s: i64) -> usize {
        let mut lower = 0;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
