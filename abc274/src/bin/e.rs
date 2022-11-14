use proconio::input;

fn main() {
    input! {
        mut n:usize,
        m:usize,
        xy:[(f64,f64);n],
        pq:[(f64,f64);m],
    }
    let mut p = xy.iter().chain(pq.iter()).collect::<Vec<_>>();
    p.insert(0, &(0.0, 0.0));
    n += 1;
    let l = p.len();
    let inf = 1e15;
    let mut dp = vec![vec![inf; l]; 1 << l];
    dp[0][0] = 0.0;
    let guide = (1 << l) - (1 << n);
    for i in 0usize..1 << l {
        for j in 0..l {
            for k in 0..l {
                if i >> k & 1 == 1 {
                    // 訪問済み
                    continue;
                }
                let cost = (p[j].0 - p[k].0).hypot(p[j].1 - p[k].1)
                    / (1 << (i & guide).count_ones()) as f64;
                dp[i | 1 << k][k] = min(dp[i | 1 << k][k], dp[i][j] + cost);
            }
        }
    }
    let mut ans = inf;
    for i in (((1 << n) - 1)..1 << l).step_by(1 << n) {
        ans = min(ans, dp[i][0]);
    }
    println!("{}", ans);
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
