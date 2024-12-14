use std::collections::BTreeMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:[usize;5],
    }
    let mut map = BTreeMap::new();
    for i in 1..32 {
        let mut pnt = 0;
        let mut name = vec![];
        for j in 0..5 {
            if i >> j & 1 == 1 {
                pnt += a[j];
                name.push((b'A' + j as u8) as char);
            }
        }
        map.entry(pnt).or_insert(vec![]).push(name);
    }
    for (_, mut v) in map.into_iter().rev() {
        v.sort();
        for v in v {
            println!("{}", v.iter().join(""));
        }
    }
}
