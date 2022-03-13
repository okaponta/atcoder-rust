use itertools::Itertools;
use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
       n:usize,
       a:[usize;n],
       b:[usize;n],
    }
    let mut ab = a.into_iter().zip(b).collect_vec();
    ab.sort();
    let mut ans = 0;
    // dp[i] = Bのsumがiの場合の数
    let mut dp = vec![1; 5001];
    for i in 0..n {
        let (ai, bi) = ab[i];
        let mut next = dp.clone();
        for j in 0..=5000 - bi {
            next[j + bi] = (next[j + bi] + dp[j]) % MOD;
        }
        if !(ai < bi) {
            ans = (ans + dp[ai - bi]) % MOD;
        }
        dp = next;
    }
    println!("{}", ans);
}
