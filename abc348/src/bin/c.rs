use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        ac:[(usize,usize);n],
    }
    let mut map = HashMap::new();
    for (a, c) in ac {
        map.insert(c, *map.get(&c).unwrap_or(&(1 << 60)).min(&a));
    }
    let mut ans = 0;
    for (_, v) in map {
        ans = ans.max(v);
    }
    println!("{}", ans);
}
