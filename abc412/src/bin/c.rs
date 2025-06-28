#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        test();
    }
}

fn test() {
    input! {
        n:usize,
        s:[usize;n],
    }
    let mut ans = 2;
    let mut cur = s[0];
    let target = s[n - 1];
    let mut set = BTreeSet::new();
    for i in 1..n - 1 {
        set.insert(s[i]);
    }
    loop {
        if target <= cur * 2 {
            break;
        }
        if let Some(&next) = set.range(..=cur * 2).last() {
            if next == cur {
                println!("-1");
                return;
            }
            ans += 1;
            cur = next;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
