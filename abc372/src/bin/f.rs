use std::collections::{HashSet, VecDeque};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        xy:[(Usize1,Usize1);m],
    }
    let mut q = (0..n).into_iter().map(|_i| 0usize).collect::<VecDeque<_>>();
    q[0] += 1;
    let mut set = HashSet::new();
    let mut g = vec![vec![]; n];
    for (x, y) in xy {
        g[x].push(y);
        set.insert(x);
    }
    for _ in 0..k {
        let mut add = vec![];
        for &i in &set {
            if q[i] != 0 {
                for &next in &g[i] {
                    add.push((next, q[i]));
                }
            }
        }
        q.rotate_right(1);
        for (i, v) in add {
            q[i] += v;
            q[i] %= MOD;
        }
    }
    let mut ans = 0;
    while let Some(v) = q.pop_front() {
        ans += v;
        ans %= MOD;
    }
    println!("{ans}");
}
