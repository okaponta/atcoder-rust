use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        d:usize,
        a:[usize;n],
    }
    let mut dp = vec![vec![None; d]; n + 1];
    dp[0][0] = Some(0);
    for i in 0..n {
        for j in (0..=i).rev() {
            for k in 0..d {
                if let Some(bef) = dp[j][k] {
                    if let Some(mx) = dp[j + 1][(k + a[i]) % d] {
                        dp[j + 1][(k + a[i]) % d] = Some(mx.max(bef + a[i]));
                    } else {
                        dp[j + 1][(k + a[i]) % d] = Some(bef + a[i]);
                    }
                }
            }
        }
    }
    println!(
        "{}",
        if let Some(ans) = dp[k][0] {
            ans as i64
        } else {
            -1
        }
    );
}
