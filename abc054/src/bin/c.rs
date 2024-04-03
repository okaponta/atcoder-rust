use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dp = vec![vec![0; n]; 1 << n];
    dp[1][0] = 1;
    for i in 1..1 << n {
        for j in 0..n {
            if i >> j & 1 != 1 {
                continue;
            }
            for &next in &g[j] {
                if i >> next & 1 == 1 {
                    continue;
                }
                dp[i | 1 << next][next] += dp[i][j];
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += dp[(1 << n) - 1][i];
    }
    println!("{}", ans);
}
