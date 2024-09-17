use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mg:usize,
        uv:[(Usize1,Usize1);mg],
        mh:usize,
        ab:[(Usize1,Usize1);mh],
    }
    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {ai:[usize;n-i-1]}
        for j in 0..n - i - 1 {
            a[i][j + i + 1] = ai[j];
            a[j + i + 1][i] = ai[j];
        }
    }
    let mut ge = HashSet::new();
    let mut he = HashSet::new();
    for (u, v) in uv {
        ge.insert((u, v));
        ge.insert((v, u));
    }
    for (a, b) in ab {
        he.insert((a, b));
        he.insert((b, a));
    }
    let mut ans = 1 << 60;
    for p in (0..n).into_iter().permutations(n) {
        let mut tmp = 0;
        for i in 0..n {
            for j in i..n {
                let c1 = ge.contains(&(i, j));
                let c2 = he.contains(&(p[i].min(p[j]), p[i].max(p[j])));
                if c1 ^ c2 {
                    tmp += a[p[i]][p[j]];
                }
            }
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
