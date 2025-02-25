use itertools::*;
use proconio::{marker::*, *};
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
        c:[Chars;n],
    }
    let mut q = VecDeque::new();
    let mut ans = vec![vec![1 << 60; n]; n];
    let mut before = vec![vec![vec![]; 26]; n];
    let mut after = vec![vec![vec![]; 26]; n];
    for i in 0..n {
        for j in 0..n {
            if c[i][j] != '-' {
                let cc = (c[i][j] as u8 - b'a') as usize;
                q.push_back((i, j, 1, true, cc));
                ans[i][j] = 1;
                before[j][cc].push(i);
                after[i][cc].push(j);
            }
        }
    }
    while let Some((i, j, len, same, cc)) = q.pop_front() {
        if same {
            for &k in &before[i][cc] {
                if ans[k][j] > len + 1 {
                    ans[k][j] = len + 1;
                    q.push_back((k, j, len + 1, true, cc));
                }
            }
            for &k in &after[j][cc] {
                if ans[i][k] > len + 1 {
                    ans[i][k] = len + 1;
                    q.push_back((i, k, len + 1, true, cc));
                }
            }
        }
        for k in 0..26 {
            for &a in &before[i][k] {
                for &b in &after[j][k] {
                    if ans[a][b] > len + 2 {
                        ans[a][b] = len + 2;
                        q.push_back((a, b, len + 2, false, 99));
                    }
                }
            }
        }
    }
    for i in 0..n {
        println!(
            "{}",
            (0..n).into_iter().map(|j| f(i, j, ans[i][j])).join(" ")
        );
    }
}

fn f(i: usize, j: usize, ans: usize) -> i64 {
    if i == j {
        return 0;
    }
    if ans == 1 << 60 {
        -1
    } else {
        ans as i64
    }
}
