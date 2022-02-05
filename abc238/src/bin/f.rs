use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,k:usize,
        p:[Usize1;n],
        q:[usize;n],
    }
    let mut result = vec![0; n];
    for i in 0..n {
        result[p[i]] = q[i];
    }
    // dp[i][j] i:これまで選んだ数字の数、j:選ばなかった最小の順位
    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0][0] = 1;
    for cur in 0..n {
        let mut next = vec![vec![0; n + 1]; k + 1];
        for i in 0..=k {
            for j in 0..=n {
                let c = result[cur];
                // 選んだ場合
                if i < k && (c < j || j == 0) {
                    // k以上選択できない && これまでの最小より順位がよくないと選択できない
                    next[i + 1][j] += dp[i][j];
                    next[i + 1][j] %= MOD;
                }
                // 選ばない場合
                if c < j || j == 0 {
                    next[i][c] += dp[i][j];
                    next[i][c] %= MOD;
                } else {
                    next[i][j] += dp[i][j];
                    next[i][j] %= MOD;
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for &e in &dp[k] {
        ans += e;
        ans %= MOD;
    }
    println!("{}", ans);
}
