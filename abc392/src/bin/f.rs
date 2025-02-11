#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    let mut ans = vec![0; n];
    let mut fw = FenwickTree::new(n);
    for i in 0..n {
        fw.add(i + 1, 1);
    }
    for i in (0..n).rev() {
        let pos = fw.lower(p[i] as i64) as usize - 1;
        ans[pos] = i + 1;
        fw.update(pos + 1, 0);
    }
    println!("{}", ans.iter().join(" "));
}

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
            if self.sum(med) < s {
                lower = med;
            } else {
                upper = med;
            }
        }
        upper
    }
}
