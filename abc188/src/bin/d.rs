use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,cc:i64,
       abc:[(i64,i64,i64);n],
    }
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for (a, b, c) in abc {
        *map.entry(a).or_insert(0) += c;
        *map.entry(b + 1).or_insert(0) -= c;
        set.insert(a);
        set.insert(b + 1);
    }
    let mut index = set.iter().collect_vec();
    index.sort();
    let mut ans = 0;
    let mut d = 0;
    let mut bef = 0;
    for &i in index {
        ans += d.min(cc) * (i as i64 - bef);
        d += map.get(&i).unwrap();
        bef = i as i64;
    }
    println!("{}", ans);
}
