use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};
use std::collections::HashSet;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    let mut rem = vec![];
    for i in 0..m {
        if !uf.union(ab[i].0, ab[i].1) {
            rem.push(i);
        }
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }
    println!("{}", set.len() - 1);
    let mut ans = vec![];
    for i in rem {
        if set.len() == 1 {
            break;
        }
        let a = uf.find(ab[i].0);
        set.remove(&a);
        let b = *set.iter().next().unwrap();
        uf.union(a, b);
        set.remove(&b);
        ans.push((i + 1, ab[i].0 + 1, b + 1));
        set.insert(uf.find(a));
    }
    for (a, b, c) in ans {
        println!("{} {} {}", a, b, c);
    }
}
