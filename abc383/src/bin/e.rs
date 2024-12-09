use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        mut uvw:[(Usize1,Usize1,usize);m],
        a:[Usize1;k],
        b:[Usize1;k],
    }
    uvw.sort_by_key(|e| e.2);
    let mut cnt_a = vec![0; n];
    let mut cnt_b = vec![0; n];
    for i in 0..k {
        cnt_a[a[i]] += 1;
        cnt_b[b[i]] += 1;
    }
    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for (u, v, w) in uvw {
        if uf.equiv(u, v) {
            continue;
        }
        let bu = uf.find(u);
        let bv = uf.find(v);
        uf.union(u, v);
        let after = uf.find(u);
        cnt_a[after] = cnt_a[bu] + cnt_a[bv];
        cnt_b[after] = cnt_b[bu] + cnt_b[bv];
        let new = cnt_a[after].min(cnt_b[after]);
        cnt_a[after] -= new;
        cnt_b[after] -= new;
        ans += new * w;
    }
    println!("{}", ans);
}
