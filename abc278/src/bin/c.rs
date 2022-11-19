use std::collections::{HashMap, HashSet};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        _n:usize,
        q:usize,
        tab:[(u8,Usize1,Usize1);q],
    }
    let mut map = HashMap::new();
    for (t, a, b) in tab {
        if t == 1 {
            (*map.entry(a).or_insert(HashSet::new())).insert(b);
        } else if t == 2 {
            (*map.entry(a).or_insert(HashSet::new())).remove(&b);
        } else {
            if map.contains_key(&a) && map.contains_key(&b) {
                println!(
                    "{}",
                    if map[&a].contains(&b) && map[&b].contains(&a) {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            } else {
                println!("No");
            }
        }
    }
}
