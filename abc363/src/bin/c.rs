use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }
    let mut set = HashSet::new();
    for v in (0..n).into_iter().permutations(n) {
        let target = v.iter().map(|&i| s[i]).collect::<Vec<_>>();
        if set.contains(&target) {
            continue;
        }
        let mut flg = true;
        for j in 0..=n - k {
            if (0..k).all(|i| target[j + i] == target[j + k - 1 - i]) {
                flg = false;
            }
        }
        if flg {
            set.insert(target);
        }
    }
    println!("{}", set.len());
}
