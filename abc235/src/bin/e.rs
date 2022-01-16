use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,q:usize,
       mut abc:[(Usize1,Usize1,usize);m],
       mut uvw:[(Usize1,Usize1,usize);q],
    }
    let mut es = vec![];
    for (a, b, c) in abc {
        es.push((c, 0, a, b));
    }
    for (i, (u, v, w)) in uvw.into_iter().enumerate() {
        es.push((w, i + 1, u, v));
    }
    es.sort();
    let mut uf = UnionFind::new(n);
    let mut ans = vec!["No"; q];
    for (_, i, a, b) in es {
        if i == 0 {
            uf.union(a, b);
        } else if !uf.equiv(a, b) {
            // クエリ対象、最小全域木の対象になるならYes
            ans[i - 1] = "Yes";
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
