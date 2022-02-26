use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        qn:usize,
    }
    let mut s = BTreeSet::new();
    for i in 0..qn {
        input! {query:i32}
        match query {
            1 => {
                input! {x: i64}
                s.insert((x, i));
            }
            2 => {
                input! {x: i64 , k:usize}
                let ans = s.range(..(x, qn)).rev().nth(k - 1).map_or(-1, |p| p.0);
                println!("{}", ans);
            }
            _ => {
                input! {x: i64 , k:usize}
                let ans = s.range((x, 0)..).nth(k - 1).map_or(-1, |p| p.0);
                println!("{}", ans);
            }
        }
    }
}
