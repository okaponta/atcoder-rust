use itertools::iproduct;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:i64,
        a:[i64;n],
    }
    let e = iproduct!(0..n, 0..n)
        .into_iter()
        .map(|(i, j)| (i, j, (pow(a[i], a[j], m) + pow(a[j], a[i], m)) % m))
        .collect::<Vec<_>>();
    println!("{}", kruskal(n, e));
}

fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by_key(|e| -e.2);
    let mut uf = UnionFind::new(n);
    let mut res = 0;
    for (u, v, cost) in edges {
        if uf.union(u, v) {
            res += cost;
        }
    }
    res
}

fn pow(mut x: i64, mut n: i64, modulo: i64) -> i64 {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
