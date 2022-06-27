use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
       h:usize,
       w:usize,
       a:[Chars;h],
    }
    let mut ans = vec![vec![1 << 30; w]; h];
    let mut map = HashMap::new();
    let mut goal = (0, 0);
    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                q.push_back((i, j, 0));
            }
            if a[i][j] == 'G' {
                goal = (i, j);
            }
            if a[i][j].is_ascii_lowercase() {
                (*map.entry(a[i][j]).or_insert(vec![])).push((i, j));
            }
        }
    }
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    while let Some((i, j, d)) = q.pop_front() {
        if ans[i][j] <= d {
            continue;
        }
        ans[i][j] = d;
        if a[i][j].is_ascii_lowercase() {
            if map.contains_key(&a[i][j]) {
                for &(ii, jj) in map.get(&a[i][j]).unwrap() {
                    if !(i == ii && j == jj) {
                        q.push_back((ii, jj, d + 1));
                    }
                }
                map.remove(&a[i][j]);
            }
        }
        for k in 0..4 {
            let x = i as i32 + dx[k];
            let y = j as i32 + dy[k];
            if 0 <= x && x < h as i32 && 0 <= y && y < w as i32 {
                let x = x as usize;
                let y = y as usize;
                if a[x][y] != '#' && d < ans[x][y] {
                    q.push_back((x, y, d + 1));
                }
            }
        }
    }
    if ans[goal.0][goal.1] == 1 << 30 {
        println!("-1");
    } else {
        println!("{}", ans[goal.0][goal.1]);
    }
}
