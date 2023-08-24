use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut dp = vec![vec![1e9; 31]; n];
    dp[0][0] = 0.0;
    for i in 0..n {
        for j in 0..31 {
            for k in 1..31 - j {
                if n <= i + k {
                    break;
                }
                let tmp = dp[i][j] + d(xy[i], xy[i + k]);
                if tmp < dp[i + k][j + k - 1] {
                    dp[i + k][j + k - 1] = tmp;
                }
            }
        }
    }
    let mut ans = dp[n - 1][0];
    for i in 1..31 {
        let tmp = dp[n - 1][i] + 2i64.pow(i as u32 - 1) as f64;
        if tmp < ans {
            ans = tmp;
        }
    }
    println!("{}", ans);
}

fn d(p: (i64, i64), q: (i64, i64)) -> f64 {
    ((p.0 - q.0) as f64).hypot((p.1 - q.1) as f64)
}
