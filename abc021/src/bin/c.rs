use std::collections::{HashSet, VecDeque};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        a:Usize1,
        b:Usize1,
        m:usize,
        xy:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for &(x, y) in &xy {
        g[x].push(y);
        g[y].push(x);
    }
    let mut d = vec![!0; n];
    let mut c = vec![0; n];
    let mut q = VecDeque::new();
    q.push_back((0usize, a, b));
    c[a] = 1;
    d[a] = 0;
    let mut used = HashSet::new();
    while let Some((dist, cur, from)) = q.pop_front() {
        if d[cur] < dist {
            continue;
        }
        c[cur] += c[from];
        c[cur] %= MOD;
        for &next in &g[cur] {
            if d[next] < dist + 1 {
                continue;
            }
            if used.contains(&(cur, next)) {
                continue;
            }
            d[next] = dist + 1;
            used.insert((cur, next));
            q.push_back((dist + 1, next, cur));
        }
    }
    println!("{}", c[b]);
}
