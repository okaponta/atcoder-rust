use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut g = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            d:[usize;n-i-1],
        }
        for j in 0..n - i - 1 {
            g[i][i + j + 1] = d[j];
            g[i + j + 1][i] = d[j];
        }
    }
    let mut dp = vec![0; 1 << n];
    // i が今まで使った辺
    for i in 0..1 << n {
        // jがこれから使う辺ひとつめ
        for j in 0..n {
            if i >> j & 1 == 1 {
                continue;
            }
            // kがこれから使う辺ふたつめ
            for k in 0..n {
                if j == k {
                    continue;
                }
                if i >> k & 1 == 1 {
                    continue;
                }
                // チェック
                dp[i | 1 << j | 1 << k] = dp[i | 1 << j | 1 << k].max(dp[i] + g[j][k]);
            }
        }
    }
    let mut ans = 0;
    for i in 0..1 << n {
        ans = ans.max(dp[i]);
    }
    println!("{}", ans);
}
