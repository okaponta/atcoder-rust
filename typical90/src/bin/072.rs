use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                q.push_back((i, j, vec![(i, j)]));
            }
        }
    }
    let mut ans = 0;
    while let Some((i, j, path)) = q.pop_front() {
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            if path[0] == (ni, nj) {
                ans = ans.max(path.len());
                continue;
            }
            if c[ni][nj] == '.' && !path.contains(&(ni, nj)) {
                let mut npath = path.clone();
                npath.push((ni, nj));
                q.push_back((ni, nj, npath));
            }
        }
    }
    if ans < 3 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
