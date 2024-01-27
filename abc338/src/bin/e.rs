use crate::fenwick::FenwickTree;
use proconio::input;
use std::cmp::Ordering::{Greater, Less};

mod fenwick {
    #[derive(Debug)]
    pub struct FenwickTree {
        tree: Vec<i64>,
    }

    impl FenwickTree {
        pub fn new(n: usize) -> Self {
            Self {
                tree: vec![0; n + 1],
            }
        }

        pub fn add(&mut self, mut i: usize, x: i64) {
            i += 1;
            while i < self.tree.len() {
                self.tree[i] += x;
                i += i & i.wrapping_neg();
            }
        }

        pub fn sum(&self, mut i: usize) -> i64 {
            i += 1;
            let mut result = 0;
            while i > 0 {
                result += self.tree[i];
                i ^= i & i.wrapping_neg();
            }
            result
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }
    for i in 0..n {
        if lr[i].1 < lr[i].0 {
            lr[i] = (lr[i].1, lr[i].0);
        }
    }

    lr.sort_by(|a, b| {
        if a.0 < b.0 {
            Less
        } else if a.0 == b.0 {
            if a.1 < b.1 {
                Greater
            } else {
                Less
            }
        } else {
            Greater
        }
    });
    let mut fwt = FenwickTree::new(2 * n + 1);

    let mut ans = 0;
    for (l, r) in &lr {
        ans += fwt.sum(*r - 1) - fwt.sum(*l);
        fwt.add(*r, 1);
    }

    println!("{}", if 0 < ans { "Yes" } else { "No" });
}
