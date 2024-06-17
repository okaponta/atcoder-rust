use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
        s:(Usize1,Usize1),
        a:[[i64;w];h],
    }
    if k < 1500 {
        let dp = f(h, w, k, s, &a);
        let mut ans = 0;
        for i in 0..h {
            for j in 0..w {
                ans = ans.max(dp[i][j]);
            }
        }
        println!("{}", ans);
        return;
    }
    let dp = f(h, w, 1500, s, &a);
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(dp[i][j] + a[i][j] * (k as i64 - 1500));
        }
    }
    println!("{}", ans);
}

fn f(h: usize, w: usize, k: usize, s: (usize, usize), a: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut dp = vec![vec![-1 << 60; w]; h];
    dp[s.0][s.1] = 0;
    for _ in 0..k {
        let mut next = vec![vec![-1 << 60; w]; h];
        for i in 0..h {
            for j in 0..w {
                for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0), (0, 0)] {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if h <= ni || w <= nj {
                        continue;
                    }
                    next[ni][nj] = next[ni][nj].max(dp[i][j] + a[ni][nj]);
                }
            }
        }
        dp = next;
    }
    dp
}
