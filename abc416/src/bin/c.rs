#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use std::cmp::Reverse;

fn main() {
    input! {
        n:usize,
        k:usize,
        x:Usize1,
        s:[Chars;n],
    }
    let mut heap: BinaryHeap<_> = BinaryHeap::new();
    heap.push(Reverse(vec![]));
    for _ in 0..k {
        let mut next = BinaryHeap::new();
        while let Some(Reverse(ss)) = heap.pop() {
            for i in 0..n {
                let mut sss = ss.clone();
                for &j in &s[i] {
                    sss.push(j);
                }
                next.push(Reverse(sss));
            }
        }
        heap = next;
    }
    for _ in 0..x {
        heap.pop();
    }
    if let Some(Reverse(s)) = heap.pop() {
        println!("{}", s.iter().join(""));
    }
}
