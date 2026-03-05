#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        mut s:Chars,
        mut t:Chars,
    }
    if s.iter().filter(|c| c != &&'A').collect::<String>()
        != t.iter().filter(|c| c != &&'A').collect::<String>()
    {
        println!("-1");
        return;
    }
    s.push('A');
    t.push('A');
    s.insert(0, 'A');
    t.insert(0, 'A');
    let sa = to_a_len(s);
    let ta = to_a_len(t);
    let mut ans = 0;
    for i in 0..sa.len() {
        ans += (sa[i] - ta[i]).abs();
    }
    println!("{}", ans);
}

fn to_a_len(s: Vec<char>) -> Vec<i32> {
    let mut res = vec![];
    let mut count = 0;
    for c in s {
        if c == 'A' {
            count += 1;
        } else {
            res.push(count);
            count = 0;
        }
    }
    res.push(count);
    res
}
