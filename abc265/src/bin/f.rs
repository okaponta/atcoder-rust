use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        d:usize,
        p:[i64;n],
        q:[i64;n],
    }
    // dp[i][j] = pからの距離i, qからの距離j
    let mut dp = vec![vec![0; d + 1]; d + 1];
    dp[0][0] = 1;
    for t in 0..n {
        let pt = p[t];
        let qt = q[t];
        let s = (pt - qt).abs() as usize;

        let mut next = vec![vec![0; d + 1]; d + 1];

        // 左上から右下
        let mut dp2 = vec![vec![0; d + 1]; d + 1];
        for i in 0..=d {
            for j in 0..=d {
                dp2[i][j] = dp[i][j];
                if i != 0 && j != d {
                    dp2[i][j] += dp2[i - 1][j + 1];
                    dp2[i][j] %= MOD;
                }
            }
        }
        for i in 0..=d {
            for j in 0..=d {
                // 終点
                let mut si = i as i64;
                let mut sj = j as i64 - s as i64;
                if sj < 0 {
                    si = i as i64 + j as i64 - s as i64;
                    sj = 0;
                }
                if 0 <= si && si <= d as i64 && 0 <= sj && sj <= d as i64 {
                    next[i][j] += dp2[si as usize][sj as usize];
                    next[i][j] %= MOD;
                }
                // 始点
                let ti = i as i64 - (s + 1) as i64;
                let tj = j as i64 + 1;
                if 0 <= ti && ti <= d as i64 && 0 <= tj && tj <= d as i64 {
                    next[i][j] = next[i][j] + MOD - dp2[ti as usize][tj as usize];
                    next[i][j] %= MOD;
                }
            }
        }

        // 左下から右上
        let mut dp3 = vec![vec![0; d + 1]; d + 1];
        for i in 0..=d {
            for j in 0..=d {
                dp3[i][j] = dp[i][j];
                if i != 0 && j != 0 {
                    dp3[i][j] += dp3[i - 1][j - 1];
                    dp3[i][j] %= MOD;
                }
                if i + 1 <= d && j + s + 1 <= d {
                    // +1しないと境界が2重に数えられる
                    // dp2と同じく貰うdpでやると負数の処理が煩雑になるので配るdpをつかっている
                    next[i + 1][j + s + 1] += dp3[i][j];
                    next[i + 1][j + s + 1] %= MOD;
                }
                if i + s + 1 <= d && j + 1 <= d {
                    next[i + s + 1][j + 1] += dp3[i][j];
                    next[i + s + 1][j + 1] %= MOD;
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 0..=d {
        for j in 0..=d {
            ans += dp[i][j];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
