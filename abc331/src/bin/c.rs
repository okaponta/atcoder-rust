use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut b = vec![];
    for i in 0..n {
        b.push(a[i]);
    }
    b.sort();
    b.reverse();
    let mut map = HashMap::new();
    let mut tmp = 0;
    for i in 0..n {
        if !map.contains_key(&b[i]) {
            map.insert(b[i], tmp);
        }
        tmp += b[i];
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(map[&a[i]]);
    }
    println!("{}", ans.iter().join(" "));
}
