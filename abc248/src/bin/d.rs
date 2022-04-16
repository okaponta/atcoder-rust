use std::collections::HashMap;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        q:usize,
        lrx:[(Usize1,Usize1,usize);q],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        let entry = map.entry(a[i]).or_insert(vec![]);
        entry.push(i);
    }
    for (l, r, x) in lrx {
        if !map.contains_key(&x) {
            println!("0");
            continue;
        }
        let v = map.get(&x).unwrap();
        let ll = v.lower_bound(&l);
        let rr = v.upper_bound(&r);
        println!("{}", rr - ll);
    }
}
