use itertools::*;
use proconio::{marker::*, *};
use std::collections::VecDeque;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n:usize,
        c:[Chars;n],
    }
    let mut q = VecDeque::new();
    let mut ans = vec![vec![1 << 60; n]; n];
    for i in 0..n {
        q.push_back((i, i, 0));
        ans[i][i] = 0;
    }
    for (i, j) in iproduct!(0..n, 0..n) {
        if c[i][j] != '-' {
            q.push_back((i, j, 1));
            ans[i][j] = ans[i][j].min(1);
        }
    }
    while let Some((i, j, len)) = q.pop_front() {
        for (k, l) in iproduct!(0..n, 0..n) {
            if c[k][i] == c[j][l] && c[k][i] != '-' && len + 2 < ans[k][l] {
                ans[k][l] = len + 2;
                q.push_back((k, l, len + 2));
            }
        }
    }
    let f = |x| if x == INF { -1 } else { x as i64 };
    for i in 0..n {
        println!("{}", (0..n).into_iter().map(|j| f(ans[i][j])).join(" "));
    }
}
