use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[f64;n],
    }
    let mut dp = vec![-1500.0; n + 1];
    dp[0] = 0.0;
    for i in 0..n {
        for j in (0..=i).rev() {
            let tmp = dp[j] * 0.9 + p[i];
            if dp[j + 1] < tmp {
                dp[j + 1] = tmp;
            }
        }
    }
    let mut ans = -1500.0;
    let mut div = vec![1.0; n + 1];
    for i in 2..=n {
        div[i] = div[i - 1] * 0.9 + 1.0;
    }
    for i in 1..=n {
        let tmp = dp[i] / div[i] - 1200.0 / (i as f64).sqrt();
        if ans < tmp {
            ans = tmp;
        }
    }
    println!("{}", ans);
}
