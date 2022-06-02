use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    let mut dp = vec![0i64; 3];
    for val in map.values() {
        let mut next = dp.clone();
        next[0] += val;
        next[1] += dp[0] * val;
        next[2] += dp[1] * val;
        dp = next;
    }
    println!("{}", dp[2]);
}
