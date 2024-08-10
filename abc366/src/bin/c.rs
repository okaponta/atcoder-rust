use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {qu: u8}
        if qu == 1 {
            input! {x: usize}
            *map.entry(x).or_insert(0) += 1;
        } else if qu == 2 {
            input! {x: usize}
            if map[&x] == 1 {
                map.remove(&x);
            } else {
                *map.entry(x).or_insert(0) -= 1;
            }
        } else {
            println!("{}", map.len());
        }
    }
}
