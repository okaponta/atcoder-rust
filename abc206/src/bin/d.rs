use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
       n:usize,a:[usize;n],
    }
    let mut count = 0;
    let mut uf = UnionFind::new(200001);
    for i in 0..n / 2 {
        if uf.union(a[i], a[n - i - 1]) {
            count += 1;
        }
    }
    println!("{}", count);
}
