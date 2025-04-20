#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        k:usize,
        mut s:Chars,
    }
    for i in 0..n {
        if 0 < i && s[i] == 'o' {
            s[i - 1] = '.';
        }
        if i < n - 1 && s[i] == 'o' {
            s[i + 1] = '.';
        }
    }
    let r = run_length_encode(s.clone());
    let mut count = 0;
    for &(c, l) in &r {
        if c == 'o' {
            count += 1;
        }
        if c == '?' {
            count += (l + 1) / 2;
        }
    }
    if count == k {
        let mut i = 0;
        for &(c, l) in &r {
            if c == '?' && l % 2 == 1 {
                for j in 0..l {
                    if j % 2 == 0 {
                        s[i + j] = 'o';
                    } else {
                        s[i + j] = '.';
                    }
                }
            }
            i += l;
        }
    }
    let cnt = (0..n).filter(|&i| s[i] == 'o').count();
    if cnt == k {
        for i in 0..n {
            if s[i] == '?' {
                s[i] = '.';
            }
        }
    }
    println!("{}", s.iter().join(""));
}

fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
