#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {t:usize}
    (0..t).into_iter().for_each(|_| case());
}

fn case() {
    input! {n:usize, s:Chars}
    let mut c = vec![0; n + 1];
    let mut sum = 0;
    let mut max = 0;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == '1' {
            sum += 1;
        }
        c[i + 1] = c[i] + if s[i] == '0' { 1 } else { -1 };
        ans = ans.min(c[i + 1] - max);
        max = max.max(c[i + 1]);
    }
    println!("{}", sum + ans);
}
