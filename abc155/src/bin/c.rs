use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n]
    }
    let mut map = HashMap::new();
    for si in s {
        *map.entry(si).or_insert(0) += 1;
    }
    let count = map.values().max().unwrap();
    let mut ans = map
        .iter()
        .filter(|(_, v)| v == &count)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    ans.sort();
    for s in ans {
        println!("{}", s);
    }
}
