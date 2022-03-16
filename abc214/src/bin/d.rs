use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut uvw:[(Usize1,Usize1,usize);n-1],
    }
    let mut uf = UnionFind::new(n);

    uvw.sort_by_key(|e| e.2);
    let mut count = vec![1; n];
    let mut ans = 0;
    for (u, v, w) in uvw {
        let uc = count[uf.find(u)];
        let vc = count[uf.find(v)];
        ans += w * uc * vc;
        uf.union(u, v);
        count[uf.find(v)] = uc + vc;
    }
    println!("{}", ans);
}
