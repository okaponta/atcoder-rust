use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n:usize,
        sc:[(usize,usize);n],
    }
    let mut set = BTreeSet::new();
    let mut map = HashMap::new();
    for (s, c) in sc {
        set.insert(s);
        map.insert(s, c);
    }
    let mut cur = 0;
    while let Some(&next) = set.range(cur + 1..).next() {
        let v = map[&next];
        map.insert(next, v % 2);
        if 1 < v {
            set.insert(next * 2);
            *map.entry(next * 2).or_insert(0) += v / 2;
        }
        cur = next;
    }
    let mut ans = 0;
    for (_, v) in map {
        ans += v;
    }
    println!("{}", ans);
}
