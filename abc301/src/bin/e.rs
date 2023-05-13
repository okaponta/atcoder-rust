use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        t:usize,
        a:[Chars;h],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut sweet = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            }
            if a[i][j] == 'G' {
                goal = (i, j);
            }
            if a[i][j] == 'o' {
                sweet.push((i, j));
            }
        }
    }
    sweet.insert(0, start);
    sweet.push(goal);
    let n = sweet.len();
    let mut dist = vec![vec![1 << 60; n]; n];
    for i in 0..n {
        let d = bfs(h, w, sweet[i], &a);
        for j in 0..n {
            dist[i][j] = d[sweet[j].0][sweet[j].1];
        }
    }
    let mut dp = vec![vec![1 << 60; n]; 1 << n];
    for i in 0..(n - 1) {
        dp[1 << i][i] = dist[0][i + 1];
    }
    let mut ans = -1;
    for i in 0..1 << (n - 1) {
        if i >> (n - 2) & 1 == 1 {
            // ゴールなのでスキップ
            if dp[i][n - 2] <= t {
                let tmp = i.count_ones() as i32;
                //println!("tmp: {} {}", tmp, dp[i][j]);
                ans = ans.max(tmp - 1);
            }
            continue;
        }
        // 現在のマスがj
        for j in 0..(n - 1) {
            if dp[i][j] == 1 << 60 {
                // ありえないのでスキップ
                continue;
            }
            if t < dp[i][j] {
                // 枝切り
                continue;
            }
            for k in 0..(n - 1) {
                // もし訪問済みならとばす
                if i >> k & 1 == 1 {
                    continue;
                }
                dp[i | 1 << k][k] = dp[i | 1 << k][k].min(dp[i][j] + dist[j + 1][k + 1]);
            }
        }
    }
    println!("{}", ans);
}

fn bfs(h: usize, w: usize, init: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![false; w]; h];
    let mut res = vec![vec![1 << 60; w]; h];
    let mut q = VecDeque::new();
    res[init.0][init.1] = 0usize;
    visited[init.0][init.1] = true;
    q.push_back((init.0, init.1));
    while let Some((x, y)) = q.pop_front() {
        let d = res[x][y];
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if h <= nx || w <= ny {
                continue;
            }
            if visited[nx][ny] {
                continue;
            }
            if grid[nx][ny] != '#' && d < res[nx][ny] {
                q.push_back((nx, ny));
                visited[nx][ny] = true;
                res[nx][ny] = d + 1;
            }
        }
    }
    res
}
