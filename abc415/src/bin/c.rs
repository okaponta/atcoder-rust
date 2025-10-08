#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn case() {
    input! {n:usize, mut s:Chars}
    s.insert(0, '0');
    let mut ok = vec![false; 1 << n];
    ok[0] = true;
    for i in 0..1 << n {
        if !ok[i] {
            continue;
        }
        for j in 0..n {
            if i >> j & 1 == 1 {
                continue;
            }
            if s[i | 1 << j] == '0' {
                ok[i | 1 << j] = true;
            }
        }
    }
    println!("{}", if ok[(1 << n) - 1] { "Yes" } else { "No" })
}

fn main() {
    input! {t:usize}
    for _ in 0..t {
        case();
    }
}
