use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
       qn:usize,
    }
    let mut s = BTreeSet::new();
    let mut q = VecDeque::new();
    for i in 0..qn {
        input! {query:i32}
        match query {
            1 => {
                input! {x: i32}
                q.push_back((x, i));
            }
            2 => {
                let ans = if let Some(&x) = s.iter().next() {
                    s.remove(&x);
                    x
                } else {
                    q.pop_front().unwrap()
                };
                println!("{}", ans.0);
            }
            _ => {
                while !q.is_empty() {
                    s.insert(q.pop_front().unwrap());
                }
            }
        }
    }
}
