use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        xyz:[(i64,i64,i64);n],
    }
    let sx = vec![1, 1, 1, 1, -1, -1, -1, -1];
    let sy = vec![1, 1, -1, -1, 1, 1, -1, -1];
    let sz = vec![1, -1, 1, -1, 1, -1, 1, -1];
    let mut ans = 0;
    for i in 0..8 {
        let mut dp = vec![-1 << 60; m + 1];
        dp[0] = 0;
        for j in 0..n {
            for k in (0..m.min(j + 1)).rev() {
                let tmp = sx[i] * xyz[j].0 + sy[i] * xyz[j].1 + sz[i] * xyz[j].2;
                dp[k + 1] = dp[k + 1].max(dp[k] + tmp);
            }
        }
        ans = ans.max(dp[m]);
    }
    println!("{}", ans);
}
