#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let offset = (0..n).into_iter().map(|i| n - 1 - i).collect_vec();
    let mut ans = 0;
    let mut map = HashMap::new();
    let mut map2 = HashMap::new();
    for i in 0..n {
        if let Some(&v) = map2.get(&i) {
            *map.entry(1 + offset[i]).or_insert(0) += v;
        }
        if let Some(v) = map.get(&(a[i] + offset[i])) {
            ans += v;
        }
        *map2.entry(i + 1 + a[i]).or_insert(0) += 1usize;
    }
    println!("{ans}")
}
