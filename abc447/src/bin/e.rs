#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use petgraph::unionfind::UnionFind;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut tmp = 2;
    let mut two = vec![2];
    for _ in 0..m {
        tmp = tmp * 2 % MOD;
        two.push(tmp);
    }
    let mut count = n;
    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for i in (0..m).rev() {
        if 2 < count {
            if uf.union(uv[i].0, uv[i].1) {
                count -= 1;
            }
        } else if uf.equiv(uv[i].0, uv[i].1) {
            uf.union(uv[i].0, uv[i].1);
        } else {
            ans = (ans + two[i]) % MOD;
        }
    }
    println!("{}", ans);
}
