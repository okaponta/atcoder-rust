use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[Chars;h],
        n:usize,
        rce:[(Usize1,Usize1,usize);n],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            }
            if a[i][j] == 'T' {
                goal = (i, j);
            }
        }
    }
    let mut med = vec![vec![0; w]; h];
    for &(r, c, e) in &rce {
        med[r][c] = e;
    }
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    visited.insert(start);
    while let Some((i, j)) = q.pop_front() {
        let d = bfs(h, w, (i, j), &a, '#');
        if d[goal.0][goal.1] <= med[i][j] {
            println!("Yes");
            return;
        }
        for &(r, c, _e) in &rce {
            if visited.contains(&(r, c)) {
                continue;
            }
            if d[r][c] <= med[i][j] {
                q.push_back((r, c));
                visited.insert((r, c));
            }
        }
    }
    println!("No");
}

fn bfs(
    h: usize,
    w: usize,
    init: (usize, usize),
    grid: &Vec<Vec<char>>,
    ng: char,
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
            if visited[nx][ny] || grid[nx][ny] == ng {
                continue;
            }
            q.push_back((nx, ny));
            visited[nx][ny] = true;
            res[nx][ny] = d + 1;
        }
    }
    res
}
