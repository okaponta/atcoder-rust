#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        q:usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {qi: u8}
        if qi == 1 {
            input! {x:usize}
            queue.push_back(x);
        } else {
            println!("{}", queue.pop_front().unwrap());
        }
    }
}
