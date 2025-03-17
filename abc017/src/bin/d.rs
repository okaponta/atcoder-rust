use proconio::*;
use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        mut n:usize,_m:usize,mut f:[usize;n],
    }
    n += 1;
    f.insert(0, MOD);
    let mut left = 0;
    let mut tmp = 1;
    let mut dp = vec![0; n];
    dp[0] = 1;
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 1..n {
        if let Some(j) = map.get(&f[i]) {
            while left < j[j.len() - 1] {
                tmp = (tmp + MOD - dp[left]) % MOD;
                left = (left + 1) % MOD;
            }
        }
        dp[i] = tmp;
        tmp = (tmp + dp[i]) % MOD;
        map.entry(f[i]).or_insert(vec![]).push(i);
    }
    println!("{}", dp[n - 1]);
}
