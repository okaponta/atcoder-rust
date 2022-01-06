use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       mut abc:[(Usize1,Usize1,i32);m]
    }
    abc.sort_by_key(|abc| abc.2);
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for (a, b, c) in abc {
        if !uf.union(a, b) && c > 0 {
            ans += c as i64;
        }
    }
    println!("{}", ans);
}
