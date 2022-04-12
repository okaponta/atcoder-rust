use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        p:[Usize1;n],
        q:[Usize1;n],
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.union(p[i], q[i]);
    }
    let mut count = vec![0; n];
    for i in 0..n {
        count[uf.find(i)] += 1;
    }
    if n == 1 {
        println!("1");
        return;
    }
    let mut v = vec![0i64; n + 1];
    v[1] = 1;
    v[2] = 3;
    for i in 3..=n {
        v[i] = (v[i - 1] + v[i - 2]) % 998244353;
    }
    let mut ans: i64 = 1;
    for i in 0..n {
        if count[i] == 0 {
            continue;
        }
        ans = (ans * v[count[i]]) % 998244353;
    }
    println!("{}", ans);
}
