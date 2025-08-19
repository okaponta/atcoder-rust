use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};

fn main() {
    input! {n:usize,m:usize,uvw:[(Usize1,Usize1,usize);m]}
    let mut ans = (1 << 31) - 1;
    for i in (0..31).rev() {
        ans ^= 1 << i;
        let mut uf = UnionFind::new(n);
        for &(u, v, w) in &uvw {
            if ans | w == ans {
                uf.union(u, v);
            }
        }
        if !uf.equiv(0, n - 1) {
            ans ^= 1 << i;
        }
    }
    println!("{}", ans);
}
