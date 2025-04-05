#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut map = HashMap::new();
    let mut max = 0;
    for i in 0..n {
        let s = s[i].clone();
        *map.entry(s).or_insert(0) += 1;
    }
    for (_, v) in map.iter() {
        max = max.max(*v);
    }
    for (k, v) in map.iter() {
        if *v == max {
            println!("{}", k);
            return;
        }
    }
}
