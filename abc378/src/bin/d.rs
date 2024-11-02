#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
        s:[Chars;h],
    }
    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            visited[i][j] = true;
            ans += dfs((i, j), h, w, k, &mut visited, &s);
            visited[i][j] = false;
        }
    }
    println!("{}", ans);
}

fn dfs(
    cur: (usize, usize),
    h: usize,
    w: usize,
    k: usize,
    used: &mut Vec<Vec<bool>>,
    s: &Vec<Vec<char>>,
) -> usize {
    if k == 0 {
        return 1;
    }
    let mut res = 0;
    for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
        let nx = cur.0.wrapping_add(dx);
        let ny = cur.1.wrapping_add(dy);
        if h <= nx || w <= ny {
            continue;
        }
        if used[nx][ny] || s[nx][ny] == '#' {
            continue;
        }
        used[nx][ny] = true;
        res += dfs((nx, ny), h, w, k - 1, used, s);
        used[nx][ny] = false;
    }
    res
}
