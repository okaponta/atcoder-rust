use proconio::{input, marker::Usize1};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,m:usize,k:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    for _ in 0..k {
        let mut next = vec![0; n];
        let sum = dp.iter().sum::<i64>();
        for i in 0..n {
            let mut sub = dp[i];
            for &prev in &edge[i] {
                sub += dp[prev];
            }
            next[i] = (sum - sub) % MOD;
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
