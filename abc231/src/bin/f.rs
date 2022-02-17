use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[usize;n],
       b:[usize;n],
    }
    let ca = compress(&a);
    let cb = compress(&b);

    let mut dict: HashMap<(usize, usize), i64> = HashMap::new();
    for ab in a.iter().zip(b).map(|(a, b)| (ca[&a], cb[&b])) {
        *dict.entry(ab).or_insert(0) += 1;
    }

    let mut abc: Vec<(usize, usize, i64)> = dict.into_iter().map(|((a, b), c)| (a, b, c)).collect();
    abc.sort_by(|a, b| {
        if b.0 != a.0 {
            b.0.cmp(&a.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut count = 0;
    let mut fen = FenwickTree::new(n);
    for (_, b, c) in abc {
        fen.add(b, c);
        count += fen.sum(b + 1) * c;
    }
    println!("{}", count);
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
