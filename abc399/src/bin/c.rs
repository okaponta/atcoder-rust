#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for (u, v) in uv {
        if !uf.union(u, v) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
