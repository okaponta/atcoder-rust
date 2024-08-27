use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    let ord = (0..n)
        .into_iter()
        .sorted_by_key(|&i| p[i])
        .rev()
        .map(|i| i + 1)
        .collect::<Vec<_>>();
    let mut set = BTreeSet::new();
    let mut ans = 0;
    for i in ord {
        set.insert(i);
        let x = *set.range(..i).last().unwrap_or(&0);
        let w = *set.range(..x).last().unwrap_or(&0);
        let y = *set.range(i + 1..).next().unwrap_or(&(n + 1));
        let z = *set.range(y + 1..).next().unwrap_or(&(n + 1));
        ans += ((x - w) * (y - i) + (i - x) * (z - y)) * p[i - 1];
    }
    println!("{}", ans);
}
