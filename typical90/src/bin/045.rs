use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        xy:[(i64,i64);n],
    }
    let m = 1 << n;
    let mut d = vec![0; m];
    for (i, j) in iproduct!(0..m, 0..n) {
        if i >> j & 1 != 1 {
            continue;
        }
        for k in 0..n {
            if i >> k & 1 == 1 {
                continue;
            }
            d[i | 1 << k] = d[i | 1 << k]
                .max(d[i])
                .max((xy[j].0 - xy[k].0).pow(2) + (xy[j].1 - xy[k].1).pow(2));
        }
    }
    let mut dp = vec![1 << 60; m];
    dp[0] = 0;
    for _ in 0..k {
        for i in (0..m).rev() {
            let mut t = i;
            while t > 0 {
                // もらうdp
                dp[i] = dp[i].min(d[t].max(dp[i ^ t]));
                t = t - 1 & i;
            }
        }
    }
    println!("{}", dp[m - 1]);
}
