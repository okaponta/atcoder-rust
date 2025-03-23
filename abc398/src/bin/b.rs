#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:[usize;7],
    }
    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut cnt = map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    cnt.sort();
    cnt.reverse();
    if 2 <= cnt.len() && 3 <= cnt[0] && 2 <= cnt[1] {
        println!("Yes");
        return;
    }
    println!("No");
}
