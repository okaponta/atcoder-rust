use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       n:usize,
       q:usize,
       c:[Usize1;n],
       ixy: [(u8,Usize1,Usize1);q],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        let mut m = HashMap::new();
        m.insert(c[i], 1);
        map.insert(i, m);
    }
    let mut uf = UnionFind::new(n);
    for (i, x, y) in ixy {
        if i == 1 {
            let a = uf.find(x);
            let b = uf.find(y);
            if uf.union(x, y) {
                // 結合される
                let (mas, target) = if a == uf.find(x) { (a, b) } else { (b, a) };
                for (k, v) in map.remove(&target).unwrap() {
                    *map.entry(mas)
                        .or_insert(HashMap::new())
                        .entry(k)
                        .or_insert(0) += v;
                }
            }
        } else {
            println!("{}", map[&uf.find(x)].get(&y).unwrap_or(&0));
        }
    }
}
