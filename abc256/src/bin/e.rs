use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        x:[Usize1;n],
        c:[usize;n],
    }
    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        if !uf.union(i, x[i]) {
            let mut cost = c[i];
            let mut tmp = i;
            loop {
                tmp = x[tmp];
                cost = cost.min(c[tmp]);
                if tmp == i {
                    break;
                }
            }
            ans += cost;
        }
    }
    println!("{}", ans);
}
