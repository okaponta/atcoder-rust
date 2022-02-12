use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        uf.union(a, b);
    }
    let mut count = vec![0; n];
    for i in 0..n {
        count[uf.find(i)] += 1;
    }
    println!("{}", count.iter().max().unwrap());
}
