use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
        q:usize,
    }
    let mut numbers = vec![];
    for i in 0..n {
        numbers.push(ab[i].0);
    }
    let mut queries = vec![];
    for _ in 0..q {
        input! {q: u8, x:usize}
        if q == 1 {
            input! {y:usize}
            queries.push((q, x, y));
            numbers.push(y);
        } else if q == 2 {
            input! {y:usize}
            queries.push((q, x, y));
        } else {
            queries.push((q, x, 0));
        }
    }
    // 座標圧縮
    numbers.sort();
    numbers.dedup();
    let map = compress(&numbers);
    let mut fw_num = FenwickTree::new(map.len());
    let mut fw_sum = FenwickTree::new(map.len());
    let mut deck = vec![];
    for i in 0..n {
        deck.push((ab[i].0, ab[i].1));
        fw_num.add(map[&ab[i].0], ab[i].1 as i64);
        fw_sum.add(map[&ab[i].0], (ab[i].0 * ab[i].1) as i64);
    }
    for (q, x, y) in queries {
        if q == 1 {
            let before = deck[x - 1].0;
            deck[x - 1].0 = y;
            fw_num.add(map[&before], -(deck[x - 1].1 as i64));
            fw_sum.add(map[&before], -((before * deck[x - 1].1) as i64));
            fw_num.add(map[&y], deck[x - 1].1 as i64);
            fw_sum.add(map[&y], (deck[x - 1].0 * deck[x - 1].1) as i64);
        } else if q == 2 {
            let before = deck[x - 1].1;
            deck[x - 1].1 = y;
            fw_num.add(map[&deck[x - 1].0], -(before as i64));
            fw_sum.add(map[&deck[x - 1].0], -((before * deck[x - 1].0) as i64));
            fw_num.add(map[&deck[x - 1].0], y as i64);
            fw_sum.add(map[&deck[x - 1].0], (y * deck[x - 1].0) as i64);
        } else {
            let all = fw_num.sum(map.len());
            if all < x as i64 {
                println!("-1");
            } else {
                let all_sum = fw_sum.sum(map.len());
                let pos = fw_num.lower(all - x as i64);
                let ans = all_sum - fw_sum.sum(pos)
                    + numbers[pos] as i64 * (fw_num.sum(pos) - (all - x as i64));
                println!("{}", ans);
            }
        }
    }
}

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

    pub fn upper(&self, s: i64) -> usize {
        let mut lower = 1;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        upper
    }
}
