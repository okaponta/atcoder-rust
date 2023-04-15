use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut num = vec![BTreeSet::new(); 200_001];
    let mut boxes = vec![BTreeSet::new(); n];
    for qi in 0..q {
        input! {q: u8}
        if q == 1 {
            input! {i:usize, j:Usize1}
            boxes[j].insert((i, qi));
            num[i].insert(j + 1);
        } else if q == 2 {
            input! {i:Usize1}
            println!("{}", boxes[i].iter().map(|(i, _)| i).join(" "));
        } else {
            input! {i:usize}
            println!("{}", num[i].iter().join(" "));
        }
    }
}
