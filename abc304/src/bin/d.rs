use std::collections::HashMap;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        _:usize,
        _:usize,
        n:usize,
        pq:[(Usize1,Usize1);n],
        a:usize,
        aa:[Usize1;a],
        b:usize,
        bb:[Usize1;b],
    }
    let mut map = HashMap::new();
    for (p, q) in pq {
        *map.entry((aa.lower_bound(&p), bb.lower_bound(&q)))
            .or_insert(0usize) += 1;
    }
    let mut mn = 1 << 60;
    let mut mx = 0;
    for (_, &v) in map.iter() {
        mn = mn.min(v);
        mx = mx.max(v);
    }
    let min = if map.len() != (a + 1) * (b + 1) {
        0
    } else {
        mn
    };
    println!("{} {}", min, mx);
}
