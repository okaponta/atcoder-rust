use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:(Usize1,Usize1),
        t:(Usize1,Usize1),
        a:[Chars;h],
    }
    let mut dp = vec![vec![vec![!0usize; 4]; w]; h];
    let mut q = VecDeque::new();
    for i in 0..4 {
        q.push_back((s.0, s.1, 0, i));
        dp[s.0][s.1][i] = 0;
    }
    let dir = vec![(!0, 0), (0, 1), (0, !0), (1, 0)];
    while let Some((x, y, d, i)) = q.pop_front() {
        for j in 0..4 {
            if j == i {
                let nx = x.wrapping_add(dir[j].0);
                let ny = y.wrapping_add(dir[j].1);
                if h <= nx || w <= ny || a[nx][ny] == '#' {
                    continue;
                }
                if d < dp[nx][ny][i] {
                    q.push_front((nx, ny, d, i));
                    dp[nx][ny][i] = d;
                }
            } else {
                if d + 1 < dp[x][y][j] {
                    q.push_back((x, y, d + 1, j));
                    dp[x][y][j] = d + 1;
                }
            }
        }
    }
    let mut ans = !0;
    for i in 0..4 {
        ans = ans.min(dp[t.0][t.1][i]);
    }
    println!("{}", ans);
}
