use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        abc:[(Usize1,Usize1,Usize1);n-1],
        uvw:[(Usize1,Usize1,Usize1);q],
    }
    let mut cost = 0;
    let mut g = vec![UnionFind::new(n); 10];
    for (a, b, c) in abc {
        cost += c + 1;
        for i in c..10 {
            g[i].union(a, b);
        }
    }
    for (u, v, w) in uvw {
        for i in w..10 {
            if !g[i].union(u, v) {
                cost -= i - w;
                break;
            }
        }
        println!("{}", cost);
    }
}
