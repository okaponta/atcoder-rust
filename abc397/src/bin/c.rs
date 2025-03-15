#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    *map1.entry(a[0]).or_insert(0) += 1;
    for i in 1..n {
        *map2.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = map1.len() + map2.len();
    for i in 1..n - 1 {
        *map1.entry(a[i]).or_insert(0) += 1;
        *map2.entry(a[i]).or_insert(0) -= 1;
        if map2[&a[i]] == 0 {
            map2.remove(&a[i]);
        }
        ans = ans.max(map1.len() + map2.len());
    }
    println!("{}", ans);
}
