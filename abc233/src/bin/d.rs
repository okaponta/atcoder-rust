use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
       n:usize,k:i64,
       a:[i64;n]
    }
    let mut ans: i64 = 0;
    let mut map = HashMap::new();
    let mut sum: i64 = 0;
    map.insert(0, 1);
    for ai in a {
        sum += ai;
        ans += map.get(&(sum - k)).map_or(0, |p| *p);
        *map.entry(sum).or_insert(0) += 1;
    }
    println!("{}", ans);
}
