use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _:usize,
        q:usize,
    }
    let mut next = 1;
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {qi: u8}
        if qi == 1 {
            set.insert(next);
            next += 1;
        } else if qi == 2 {
            input! {x:usize}
            set.remove(&x);
        } else {
            println!("{}", set.iter().next().unwrap());
        }
    }
}
