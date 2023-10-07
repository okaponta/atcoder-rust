use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        sc:[(usize,usize);n],
    }
    let mut map = BTreeMap::new();
    for (s, c) in sc {
        map.insert(s, c);
    }
    let mut cur = 0;
    while let Some((&s, &c)) = map.range(cur + 1..).next() {
        map.insert(s, c % 2);
        if 1 < c {
            *map.entry(s * 2).or_insert(0) += c / 2;
        }
        cur = s;
    }
    let mut ans = 0;
    for (_, v) in map {
        ans += v;
    }
    println!("{}", ans);
}
