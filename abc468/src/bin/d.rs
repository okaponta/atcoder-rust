#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        ans += calc(i, i, n, &s);
    }
    for i in 1..n {
        ans += calc(i - 1, i, n, &s);
    }
    println!("{}", ans);
}

fn calc(mut i: usize, mut j: usize, n: usize, s: &Vec<char>) -> usize {
    let mut res = 1;
    let mut diff = if eval(i, j, s) { 0 } else { 1 };
    while i != 0 && j != n - 1 && diff < 2 {
        i -= 1;
        j += 1;
        if eval(i, j, s) {
            res += 1;
        } else {
            diff += 1;
            if diff < 2 {
                res += 1;
            }
        }
    }
    return res;
}

fn eval(i: usize, j: usize, s: &Vec<char>) -> bool {
    s[i] == s[j]
}
