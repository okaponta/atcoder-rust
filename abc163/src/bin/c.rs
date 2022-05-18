use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n-1],
    }
    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    for i in 1..=n {
        println!("{}", map.get(&i).unwrap_or(&0));
    }
}
