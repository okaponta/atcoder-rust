use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n],
        m:usize,
        t:[String;m],
    }
    let mut map = HashMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }
    for t in t {
        *map.entry(t).or_insert(0) -= 1;
    }
    println!(
        "{}",
        map.into_iter().max_by_key(|(_, v)| *v).unwrap().1.max(0)
    );
}
