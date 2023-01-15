use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        st:[(String,String);n],
    }
    let mut map = HashMap::new();
    let mut count = 0;
    for i in 0..n {
        if !map.contains_key(&st[i].0) {
            map.insert(&st[i].0, count);
            count += 1;
        }
        if !map.contains_key(&st[i].1) {
            map.insert(&st[i].1, count);
            count += 1;
        }
    }
    let mut uf = UnionFind::new(count);
    for i in 0..n {
        if !uf.union(map[&st[i].0], map[&st[i].1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
