use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut ans = vec![0; n];
    ans[0] = n;
    let mut dp = HashMap::new();
    for i in 1..n {
        let mut next = dp.clone();
        for ((len, sa, nxt), val) in dp {
            if nxt == a[i] {
                *next.entry((len + 1, sa, nxt + sa)).or_insert(0) += val;
                *next.entry((len + 1, sa, nxt + sa)).or_insert(0) %= 998244353;
                ans[len + 1] += val;
                ans[len + 1] %= 998244353;
            }
        }
        for j in 0..i {
            ans[1] += 1;
            ans[1] %= 998244353;
            *next
                .entry((1usize, a[i] - a[j], 2 * a[i] - a[j]))
                .or_insert(0) += 1;
        }
        dp = next;
    }
    println!("{}", ans.iter().join(" "));
}
