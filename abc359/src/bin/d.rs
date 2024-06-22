use std::collections::HashSet;

use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }
    let mut ng = HashSet::new();
    for i in 0..1 << k {
        let mut ones = vec![];
        for j in 0..k {
            if i >> j & 1 == 1 {
                ones.push(j);
            }
        }
        if (0..ones.len()).all(|i| ones[i] + ones[ones.len() - 1 - i] == k - 1) {
            ng.insert(i);
        }
    }
    let mut dp = vec![0; 1 << k];
    let mut tmp = vec![0];
    for i in 0..k {
        let mut next = vec![];
        for j in tmp {
            if s[i] == 'A' {
                next.push(j << 1);
            } else if s[i] == 'B' {
                next.push((j << 1) + 1);
            } else {
                next.push(j << 1);
                next.push((j << 1) + 1);
            }
        }
        tmp = next;
    }
    for i in tmp {
        if !ng.contains(&i) {
            dp[i] = 1;
        }
    }
    let mask = (1 << k) - 1;
    for i in 0..n {
        if i + k == n {
            break;
        }
        let mut next = vec![0; 1 << k];
        for j in 0..1 << k {
            if s[i + k] == 'A' {
                next[(j << 1) & mask] += dp[j];
                next[(j << 1) & mask] %= MOD;
            }
            if s[i + k] == 'B' {
                next[((j << 1) & mask) + 1] += dp[j];
                next[((j << 1) & mask) + 1] %= MOD;
            }
            if s[i + k] == '?' {
                next[(j << 1) & mask] += dp[j];
                next[(j << 1) & mask] %= MOD;
                next[((j << 1) & mask) + 1] += dp[j];
                next[((j << 1) & mask) + 1] %= MOD;
            }
        }
        for &j in &ng {
            next[j] = 0;
        }
        dp = next;
    }
    let mut ans = 0;
    for i in dp {
        ans += i;
        ans %= MOD;
    }
    println!("{}", ans);
}
