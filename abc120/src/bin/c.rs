use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut q = VecDeque::new();
    let mut count = 0;
    for c in s {
        if let Some(top) = q.pop_front() {
            if top == c {
                q.push_back(top);
                q.push_back(c);
            } else {
                count += 2;
            }
        } else {
            q.push_back(c);
        }
    }
    println!("{}", count);
}
