#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,std::vec};
use std::println;

fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut a = vec![0usize; n];
    let mut ans = 0;
    let mut set = HashSet::new();
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {x:Usize1}
            ans = ans ^ a[x] ^ (a[x] + 1);
            a[x] += 1;
            println!("{ans}");
            set.insert(x);
        } else {
            let mut new_set = HashSet::new();
            for e in set {
                if 1 < a[e] {
                    new_set.insert(e);
                }
                ans = ans ^ a[e] ^ (a[e] - 1);
                a[e] -= 1;
            }
            set = new_set;
            println!("{ans}");
        }
    }
}
