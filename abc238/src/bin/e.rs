use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,q:usize,
        lr:[(usize,usize);q],
    }
    let mut uf = UnionFind::new(n + 1);
    for (l, r) in lr {
        uf.union(l - 1, r);
    }
    println!("{}", if uf.equiv(0, n) { "Yes" } else { "No" });
}
