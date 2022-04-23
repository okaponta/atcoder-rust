use proconio::input;

fn main() {
    input! {
        n:usize,p:usize,
    }
    // dp[i][j] i文字目まで考えた時にj文字である個数(mod p)
    let mut dp = vec![vec![0; n + 6]; n + 1001];
    dp[0][0] = 1;
    for i in 0..n {
        if i == 1 {
            dp[1][0] = 0;
        }
        for j in 0..=n {
            dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % p;
            let mul = if i == 0 { 26 } else { 25 };
            let next = (dp[i][j] * mul) % p;
            dp[i + 1][j + 2] = (dp[i + 1][j + 2] + next) % p;
            dp[i + 10][j + 2] = (dp[i + 10][j + 2] + p - next) % p;
            dp[i + 10][j + 3] = (dp[i + 10][j + 3] + next) % p;
            dp[i + 100][j + 3] = (dp[i + 100][j + 3] + p - next) % p;
            dp[i + 100][j + 4] = (dp[i + 100][j + 4] + next) % p;
            dp[i + 1000][j + 4] = (dp[i + 1000][j + 4] + p - next) % p;
            dp[i + 1000][j + 5] = (dp[i + 1000][j + 5] + next) % p;
        }
    }
    let mut ans = 0;
    for i in 1..n {
        ans = (ans + dp[n][i]) % p;
    }
    println!("{}", ans);
}
