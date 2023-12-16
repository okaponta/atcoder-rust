use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        tx:[(u8,Usize1);n],
    }
    let mut ans = vec![];
    let mut kmax = 0;
    let mut k = 0;
    let mut map = HashMap::new();
    for i in (0..n).rev() {
        if tx[i].0 == 1 {
            if map.contains_key(&tx[i].1) && 0 < map[&tx[i].1] {
                *map.entry(tx[i].1).or_insert(0) -= 1;
                k -= 1;
                ans.push(1);
            } else {
                ans.push(0);
            }
        } else {
            *map.entry(tx[i].1).or_insert(0) += 1;
            k += 1;
            kmax = kmax.max(k);
        }
    }
    if 0 < k {
        println!("-1");
        return;
    }
    println!("{}", kmax);
    println!("{}", ans.iter().rev().join(" "));
}
