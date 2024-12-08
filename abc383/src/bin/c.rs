use std::collections::VecDeque;

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
        d:usize,
        s:[Chars;h],
    }
    let mut q = VecDeque::new();
    let mut ans = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                q.push_back((i, j, d));
                ans[i][j] = true;
            }
        }
    }
    while let Some((i, j, d)) = q.pop_front() {
        if d == 0 {
            continue;
        }
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            if ans[ni][nj] || s[ni][nj] == '#' {
                continue;
            }
            q.push_back((ni, nj, d - 1));
            ans[ni][nj] = true;
        }
    }
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
