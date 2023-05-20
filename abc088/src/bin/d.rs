use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let dist = bfs(h, w, (0, 0), &s, '.');
    let mut white = 0;
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            white += 1;
        }
    }
    if dist[h - 1][w - 1] == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", white - dist[h - 1][w - 1] - 1);
    }
}

fn bfs(
    h: usize,
    w: usize,
    init: (usize, usize),
    grid: &Vec<Vec<char>>,
    ok: char,
) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![false; w]; h];
    let mut res = vec![vec![1 << 60; w]; h];
    let mut q = VecDeque::new();

    visited[init.0][init.1] = true;
    res[init.0][init.1] = 0usize;
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
            if grid[nx][ny] == ok && d < res[nx][ny] {
                q.push_back((nx, ny));
                visited[nx][ny] = true;
                res[nx][ny] = d + 1;
            }
        }
    }
    res
}
