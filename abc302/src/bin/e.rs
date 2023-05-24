use std::collections::{HashMap, HashSet};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {q:u8}
        if q == 1 {
            input! {u:Usize1,v:Usize1}
            (*map.entry(u).or_insert(HashSet::new())).insert(v);
            (*map.entry(v).or_insert(HashSet::new())).insert(u);
        } else {
            input! {v:Usize1}
            let rem = map.remove(&v).unwrap_or(HashSet::new());
            for i in rem {
                if map[&i].len() == 1 {
                    map.remove(&i);
                } else {
                    map.entry(i).or_default().remove(&v);
                }
            }
        }
        println!("{}", n - map.len());
    }
}
