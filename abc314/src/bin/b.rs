use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut bet = vec![HashSet::new(); n];
    for i in 0..n {
        input! {
            c:usize,
            a:[usize;c],
        }
        for a in a {
            bet[i].insert(a);
        }
    }
    input! {
        x:usize,
    }
    let mut tmp = vec![];
    let mut min = 38;
    for i in 0..n {
        if bet[i].contains(&x) {
            tmp.push((i, bet[i].len()));
            min = min.min(bet[i].len());
        }
    }
    let mut ans = vec![];
    for i in 0..tmp.len() {
        if tmp[i].1 == min {
            ans.push(tmp[i].0 + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
