use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       ab:[(Usize1,Usize1);m]
    }
    let mut road = vec![vec![]; n];
    for (a, b) in ab {
        road[a].push(b);
    }

    let mut uf = UnionFind::new(n);
    let mut ans = vec![0; n];
    for i in (1..n).rev() {
        ans[i - 1] = ans[i] + 1;
        for &v in &road[i] {
            if uf.union(i, v) {
                ans[i - 1] -= 1;
            }
        }
    }
    for e in ans {
        println!("{}", e);
    }
}
