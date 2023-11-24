use std::{collections::HashSet, mem};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        c:[Usize1;n],
        ab:[(Usize1,Usize1);q],
    }
    let mut sets = c.iter().map(|i| HashSet::from([i])).collect::<Vec<_>>();
    for (a, b) in ab {
        if sets[b].len() < sets[a].len() {
            sets.swap(a, b);
        }
        let aa = mem::take(&mut sets[a]);
        sets[b].extend(aa);
        println!("{}", sets[b].len());
    }
}
