use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       ab:[(Usize1,Usize1);m]
    }
    if n <= m {
        println!("No");
        return;
    }
    let mut neighbor = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        neighbor[a].push(b);
        neighbor[b].push(a);
        if !uf.union(a, b) {
            println!("No");
            return;
        }
    }
    for i in neighbor {
        if i.len() > 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
