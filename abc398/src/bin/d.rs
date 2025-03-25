#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        _n:usize,
        r:i64,
        c:i64,
        s:Chars,
    }
    let mut tmp = (-r, c);
    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut ans = vec![];
    for ci in s {
        match ci {
            'N' => tmp.0 -= 1,
            'E' => tmp.1 -= 1,
            'W' => tmp.1 += 1,
            _ => tmp.0 += 1,
        }
        set.insert((tmp.0 + r, tmp.1 - c));
        ans.push(if set.contains(&tmp) { '1' } else { '0' });
    }
    println!("{}", ans.iter().join(""));
}
