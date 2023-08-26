use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        abc:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut ans = 0;
    let mut dp = vec![vec![0; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for i in 0..1 << n {
        for j in 0..n {
            if i >> j & 1 == 0 {
                continue;
            }
            for &(next, cost) in &g[j] {
                if i >> next & 1 == 1 {
                    continue;
                }
                dp[i | 1 << next][next] = dp[i | 1 << next][next].max(dp[i][j] + cost);
                ans = ans.max(dp[i | 1 << next][next]);
            }
        }
    }
    println!("{}", ans);
}
