use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    let mut edges = vec![vec![]; n];
    for i in 0..m {
        uf.union(uv[i].0, uv[i].1);
        edges[uv[i].0].push(uv[i].1);
        edges[uv[i].1].push(uv[i].0);
    }
    for i in 0..n {
        if uf.find(i) != uf.find(0) {
            // NOT連結
            println!("No");
            return;
        }
    }
    let mut one = 0;
    for i in 0..n {
        if 2 < edges[i].len() {
            println!("No");
            return;
        }
        if edges[i].len() == 1 {
            one += 1;
        }
    }
    if one != 2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
