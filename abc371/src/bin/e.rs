use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut tmp = 0;
    for i in 0..n {
        tmp += i as i64 - map.get(&a[i]).unwrap_or(&-1);
        sum += tmp;
        map.insert(a[i], i as i64);
    }
    println!("{}", sum);
}
