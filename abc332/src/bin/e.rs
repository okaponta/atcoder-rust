use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        w:[usize;n],
    }
    let s = w.iter().sum::<usize>();
    // dp[i][j] = iのグッズを選び、j+1個に分けたときの最小の分散
    let mut dp = vec![vec![1e20; d]; 1 << n];
    for i in 0..1 << n {
        let mut tmp = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                tmp += w[j];
            }
        }
        dp[i][0] = dist(tmp as f64, s as f64 / d as f64);
    }
    for j in 1..d {
        for i in 0..1 << n {
            let mut q = i;
            while q > 0 {
                dp[i][j] = dp[i][j].min(dp[i - q][j - 1] + dp[q][0]);
                q = (q - 1) & i;
            }
        }
    }
    let ans = dp[(1 << n) - 1][d - 1] / d as f64;
    println!("{}", ans);
}

fn dist(a: f64, b: f64) -> f64 {
    (a - b).powi(2)
}
