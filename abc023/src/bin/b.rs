use std::collections::VecDeque;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    if n % 2 == 0 {
        println!("-1");
        return;
    }
    let mut q = VecDeque::new();
    q.push_back('b');
    let mut count = 0;
    while q.len() < n {
        if count % 3 == 0 {
            q.push_front('a');
            q.push_back('c');
        } else if count % 3 == 1 {
            q.push_front('c');
            q.push_back('a');
        } else {
            q.push_front('b');
            q.push_back('b');
        }
        count += 1;
    }
    let t = q.into_iter().collect::<Vec<_>>();
    if s == t {
        println!("{}", n / 2);
    } else {
        println!("-1");
    }
}
