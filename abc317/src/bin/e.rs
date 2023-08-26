use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        mut s:[Chars;h],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
                s[i][j] = '.';
            } else if s[i][j] == 'G' {
                goal = (i, j);
                s[i][j] = '.';
            }
        }
    }
    let mut map = vec![vec![true; w]; h];
    for i in 0..h {
        let mut flag = false;
        // > の処理
        for j in 0..w {
            if s[i][j] == '.' {
                if flag {
                    map[i][j] = false;
                }
            } else if s[i][j] == '>' {
                flag = true;
                map[i][j] = false;
            } else {
                flag = false;
                map[i][j] = false;
            }
        }
        // < の処理
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                if flag {
                    map[i][j] = false;
                }
            } else if s[i][j] == '<' {
                flag = true;
                map[i][j] = false;
            } else {
                flag = false;
                map[i][j] = false;
            }
        }
    }
    for j in 0..w {
        let mut flag = false;
        // v の処理
        for i in 0..h {
            if s[i][j] == '.' {
                if flag {
                    map[i][j] = false;
                }
            } else if s[i][j] == 'v' {
                flag = true;
                map[i][j] = false;
            } else {
                flag = false;
                map[i][j] = false;
            }
        }
        // ^ の処理
        for i in (0..h).rev() {
            if s[i][j] == '.' {
                if flag {
                    map[i][j] = false;
                }
            } else if s[i][j] == '^' {
                flag = true;
                map[i][j] = false;
            } else {
                flag = false;
                map[i][j] = false;
            }
        }
    }
    let ans = bfs(h, w, start, &map)[goal.0][goal.1];
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn bfs(h: usize, w: usize, init: (usize, usize), grid: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
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
            if grid[nx][ny] && d < res[nx][ny] {
                q.push_back((nx, ny));
                visited[nx][ny] = true;
                res[nx][ny] = d + 1;
            }
        }
    }
    res
}
