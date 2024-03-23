use std::collections::{BTreeMap, BTreeSet};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        m:usize,
        tax:[(u8,Usize1,usize);m],
    }
    let mut zero = h * w;
    let mut col = BTreeSet::new();
    let mut row = BTreeSet::new();
    let mut ans = BTreeMap::new();
    for (t, a, x) in tax.into_iter().rev() {
        if t == 1 {
            if col.contains(&a) {
                continue;
            }
            col.insert(a);
            if 0 < w - row.len() {
                *ans.entry(x).or_insert(0) += w - row.len();
                zero -= w - row.len();
            }
        } else {
            if row.contains(&a) {
                continue;
            }
            row.insert(a);
            if 0 < h - col.len() {
                *ans.entry(x).or_insert(0) += h - col.len();
                zero -= h - col.len();
            }
        }
    }
    if 0 < zero {
        *ans.entry(0).or_insert(0) += zero;
    }
    println!("{}", ans.len());
    for (k, v) in ans {
        println!("{} {}", k, v);
    }
}
