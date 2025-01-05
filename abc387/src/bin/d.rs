use std::collections::VecDeque;

use itertools::*;
use proconio::{marker::*, *};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut st = (0, 0);
    let mut gl = (0, 0);
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == 'S' {
            st = (i, j);
        }
        if s[i][j] == 'G' {
            gl = (i, j);
        }
    }
    let ans = f(h, w, st, gl, &s, true).min(f(h, w, st, gl, &s, false));
    if ans == h * w {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn f(
    h: usize,
    w: usize,
    st: (usize, usize),
    gl: (usize, usize),
    s: &Vec<Vec<char>>,
    dir: bool,
) -> usize {
    let mut g = vec![vec![h * w; w]; h];
    let mut q = VecDeque::new();
    q.push_back((st.0, st.1, 0, dir));
    while let Some((i, j, d, dir)) = q.pop_front() {
        if g[i][j] <= d {
            continue;
        }
        if (i, j) == gl {
            return d;
        }
        g[i][j] = d;
        if dir {
            for di in vec![!0, 1] {
                let ni = i.wrapping_add(di);
                if h <= ni || s[ni][j] == '#' {
                    continue;
                }
                q.push_back((ni, j, d + 1, !dir));
            }
        } else {
            for dj in vec![!0, 1] {
                let nj = j.wrapping_add(dj);
                if w <= nj || s[i][nj] == '#' {
                    continue;
                }
                q.push_back((i, nj, d + 1, !dir));
            }
        }
    }
    h * w
}
