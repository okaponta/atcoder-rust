use itertools::iproduct;
use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
       n:usize,m:usize,
       abc:[(Usize1,Usize1,usize);m]
    }
    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for (a, b, c) in abc {
        dp[a][b] = c;
    }
    let mut ans = 0;
    for k in 0..n {
        let mut e = vec![vec![0; n]; n];
        for (s, t) in iproduct!(0..n, 0..n) {
            e[s][t] = dp[s][t].min(dp[s][k] + dp[k][t]);
            if e[s][t] < INF {
                ans += e[s][t];
            }
        }
        dp = e;
    }
    println!("{}", ans);
}
