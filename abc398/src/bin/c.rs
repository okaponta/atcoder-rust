#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }
    if let Some(j) = map
        .into_iter()
        .filter(|(_, v)| 1 == *v)
        .map(|(k, _)| k)
        .max()
    {
        for i in 0..n {
            if a[i] == j {
                println!("{}", i + 1);
            }
        }
    } else {
        println!("-1");
    }
}
