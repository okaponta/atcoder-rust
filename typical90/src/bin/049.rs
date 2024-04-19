use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        clr:[(i64,Usize1,usize);m],
    }
    let g = clr
        .into_iter()
        .map(|(c, l, r)| (l, r, c))
        .collect::<Vec<_>>();
    println!("{}", kruskal(n + 1, g));
}

fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by_key(|e| e.2);
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    let mut res = 0;
    let mut count = 0;
    for (u, v, cost) in edges {
        if uf.union(u, v) {
            res += cost;
            count += 1;
        }
    }
    if count != n - 1 {
        return -1;
    }
    res
}
