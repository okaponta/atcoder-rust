use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = vec![1; 1_000_001];
    for i in 2..=1_000_000 {
        let mut count = 0;
        let mut update_keys = vec![];
        for j in (i..=1_000_000).step_by(i) {
            if map.contains_key(&j) {
                count += map[&j];
                update_keys.push(j);
            }
        }
        if k <= count {
            for j in update_keys {
                ans[j] = i;
            }
        }
    }
    for a in a {
        println!("{}", ans[a]);
    }
}
