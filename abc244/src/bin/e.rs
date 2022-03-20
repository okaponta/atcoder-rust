use proconio::{input, marker::Usize1};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,m:usize,
        k:usize,s:Usize1,t:Usize1,x:Usize1,
        uv:[(Usize1,Usize1);m]
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }
    // dp[i][j] iにいる場合の場合の数(iは0..n)
    // xが偶数の場合、j=0/奇数の場合、j=1
    let mut dp = vec![vec![0i64, 0i64]; n];
    dp[s][0] = 1;
    for _ in 0..k {
        let mut next = vec![vec![0i64, 0i64]; n];
        for i in 0..n {
            for &nexti in &edge[i] {
                if nexti == x {
                    next[nexti][0] += dp[i][1];
                    next[nexti][1] += dp[i][0];
                } else {
                    next[nexti][0] += dp[i][0];
                    next[nexti][1] += dp[i][1];
                }
                next[nexti][0] %= MOD;
                next[nexti][1] %= MOD;
            }
        }
        dp = next;
    }
    println!("{}", dp[t][0]);
}
