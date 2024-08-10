#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[[[i64;n];n];n],
        q:usize,
        lr:[(Usize1,usize,Usize1,usize,Usize1,usize);q],
    }
    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                s[i + 1][j + 1][k + 1] =
                    a[i][j][k] + s[i][j + 1][k + 1] + s[i + 1][j][k + 1] + s[i + 1][j + 1][k]
                        - s[i + 1][j][k]
                        - s[i][j + 1][k]
                        - s[i][j][k + 1]
                        + s[i][j][k];
            }
        }
    }
    for (l1, r1, l2, r2, l3, r3) in lr {
        println!("{}", f((l1, l2, l3), (r1, r2, r3), &s));
    }
}

fn f(l: (usize, usize, usize), r: (usize, usize, usize), s: &Vec<Vec<Vec<i64>>>) -> i64 {
    s[r.0][r.1][r.2] - s[l.0][r.1][r.2] - s[r.0][l.1][r.2] - s[r.0][r.1][l.2]
        + s[r.0][l.1][l.2]
        + s[l.0][r.1][l.2]
        + s[l.0][l.1][r.2]
        - s[l.0][l.1][l.2]
}
