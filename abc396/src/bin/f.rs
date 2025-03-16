#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let mut b = vec![];
    let mut target = vec![vec![]; m + 1];
    let mut inv = 0;
    let mut fw = FenwickTree::new(m);
    // 注意：1-indexedで行うこと！
    for i in 0..n {
        let bi = a[i] % m;
        inv += i as i64 - fw.sum(bi + 1);
        fw.add(bi + 1, 1);
        b.push(bi);
        target[m - bi].push(i);
    }
    for i in 0..m {
        let l = target[i].len();
        if l == 0 {
            println!("{}", inv);
            continue;
        }
        let cur = target[i].iter().sum::<usize>();
        let max = (2 * n - 1 - l) * l / 2;
        let min = l * (l - 1) / 2;
        inv += (cur * 2) as i64 - max as i64 - min as i64;
        println!("{}", inv);
    }
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
}
