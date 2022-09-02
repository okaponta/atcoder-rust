use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        txa:[(usize,usize,i64);n],
    }
    let mut map = HashMap::new();
    for (t, x, a) in txa {
        map.insert(t, (x, a));
    }
    let mut dp = vec![-1 << 60; 5];
    dp[0] = 0;
    for t in 1..100_010 {
        let mut next = dp.clone();
        for i in 0..5 {
            if i + 1 < 5 {
                next[i] = next[i].max(dp[i + 1]);
            }
            if i.wrapping_add(!0) < 5 {
                next[i] = next[i].max(dp[i - 1]);
            }
        }
        if let Some(&(x, a)) = map.get(&t) {
            next[x] += a;
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
