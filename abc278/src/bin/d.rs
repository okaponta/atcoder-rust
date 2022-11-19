use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
        q:usize,
    }
    let mut map = HashMap::new();
    let mut base = 0;
    for i in 0..n {
        map.insert(i, a[i]);
    }
    for _ in 0..q {
        input! {
            i:u8
        }
        if i == 1 {
            input! { x:usize }
            base = x;
            map = HashMap::new();
        } else if i == 2 {
            input! { i:Usize1, x:usize }
            *map.entry(i).or_insert(0) += x;
        } else {
            input! { i:Usize1 }
            println!("{}", map.get(&i).unwrap_or(&0) + base);
        }
    }
}
