use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        rr:usize,
        r:[Usize1;rr],
        abc:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let w = WarshallFloyd::new(n, &g);
    let mut dp = vec![vec![1 << 60; rr]; 1 << rr];
    for i in 0..rr {
        dp[1 << i][i] = 0;
    }
    for i in 1..1 << rr {
        for j in 0..rr {
            if i >> j & 1 != 1 {
                continue;
            }
            for k in 0..rr {
                if i >> k & 1 == 1 {
                    continue;
                }
                dp[i | 1 << k][k] = dp[i | 1 << k][k].min(dp[i][j] + w.distance[r[j]][r[k]]);
            }
        }
    }
    let mut ans = 1 << 60;
    for i in 0..rr {
        ans = ans.min(dp[(1 << rr) - 1][i]);
    }
    println!("{}", ans);
}

pub struct WarshallFloyd {
    distance: Vec<Vec<usize>>,
}

impl WarshallFloyd {
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            for &(j, c) in &edges[i] {
                distance[i][j] = c;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    // 普通に足す場合は足してこの場合はmaxをとる問題だった
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
        Self { distance }
    }
}
