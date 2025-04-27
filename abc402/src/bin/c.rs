#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut a = vec![HashSet::new(); m];
    let mut idx = vec![vec![]; n];
    for i in 0..m {
        input! {
            k:usize,
            ai:[Usize1;k],
        }
        for &ai in &ai {
            idx[ai].push(i);
            a[i].insert(ai);
        }
    }
    input! {b: [Usize1;n]}
    let mut count = 0;
    for &b in &b {
        for &i in &idx[b] {
            a[i].remove(&b);
            if a[i].is_empty() {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
