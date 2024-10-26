use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:[Chars;8],
    }
    let mut row = HashSet::new();
    let mut col = HashSet::new();
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                row.insert(i);
                col.insert(j);
            }
        }
    }
    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if row.contains(&i) || col.contains(&j) {
                continue;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
