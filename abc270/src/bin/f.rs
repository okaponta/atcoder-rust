use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        x:[usize;n],
        y:[usize;n],
        abz:[(Usize1,Usize1,usize);m],
    }
    let mut edges1 = vec![];
    let mut edges2 = vec![];
    let mut edges3 = vec![];
    let mut edges4 = vec![];
    for (a, b, z) in abz {
        edges1.push((a, b, z));
        edges2.push((a, b, z));
        edges3.push((a, b, z));
        edges4.push((a, b, z));
    }
    for i in 0..n {
        edges2.push((i, n, x[i]));
        edges4.push((i, n, x[i]));
        edges3.push((i, n, y[i]));
        edges4.push((i, n + 1, y[i]));
    }
    let mut ans = 1 << 60;
    if n - 1 <= m {
        ans = ans.min(kruskal(n, edges1));
    }
    ans = ans.min(kruskal(n + 1, edges2));
    ans = ans.min(kruskal(n + 1, edges3));
    ans = ans.min(kruskal(n + 2, edges4));
    println!("{}", ans);
}

fn kruskal(n: usize, mut edges: Vec<(usize, usize, usize)>) -> usize {
    edges.sort_by_key(|e| e.2);
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    let mut res = 0;
    for (u, v, cost) in edges {
        if uf.union(u, v) {
            res += cost;
        }
    }
    res
}
