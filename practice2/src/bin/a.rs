use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
        tuv:[(u8,usize,usize);q],
    }
    let mut uf = UnionFind::new(n);
    for (t, u, v) in tuv {
        if t == 0 {
            uf.union(u, v);
        } else {
            println!("{}", if uf.equiv(u, v) { "1" } else { "0" });
        }
    }
}
