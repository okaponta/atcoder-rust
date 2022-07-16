use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:i64,
        k:i64,
    }
    if k > n / 2 {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    let mut remain: BTreeSet<i64> = (1..=n).into_iter().collect();
    for i in 1..=n {
        let small = i - k;
        let big = i + k;
        if remain.contains(&big) && n - k < big {
            ans.push(big);
            remain.remove(&big);
        } else {
            if let Some(&x) = remain.range(..=small).next() {
                ans.push(x);
                remain.remove(&x);
            } else {
                let &x = remain.range((i + k)..).next().unwrap();
                ans.push(x);
                remain.remove(&x);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
