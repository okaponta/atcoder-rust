use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,k:usize,
       a:[Usize1;n],
    }
    let mut path = vec![0];
    let mut uf = UnionFind::new(n);
    let mut prev = 0;
    let mut next = a[0];
    while uf.union(prev, next) {
        path.push(next);
        prev = next;
        next = a[prev];
    }
    let mut begin = 0;
    for i in 0..path.len() {
        if path[i] == next {
            begin = i;
        }
    }
    let loop_size = path.len() - begin;
    let index = if k < begin {
        k
    } else {
        ((k - begin) % loop_size) + begin
    };
    println!("{}", path[index] + 1);
}
