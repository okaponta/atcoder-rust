use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _n:usize,
        a:Usize1,
        b:Usize1,
        k:usize,
        p:[Usize1;k],
    }
    let mut used = HashSet::new();
    used.insert(a);
    used.insert(b);
    for p in p {
        if used.contains(&p) {
            println!("NO");
            return;
        }
        used.insert(p);
    }
    println!("YES");
}
