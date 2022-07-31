use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(i, HashSet::new());
    }
    for (u, v) in uv {
        (*map.entry(u).or_insert(HashSet::new())).insert(v);
        (*map.entry(v).or_insert(HashSet::new())).insert(u);
    }
    let mut count = 0;
    for a in 0..n {
        for b in a..n {
            for c in b..n {
                if map[&a].contains(&b) && map[&b].contains(&c) && map[&c].contains(&a) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
