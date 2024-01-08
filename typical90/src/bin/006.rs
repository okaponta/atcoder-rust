use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }
    let mut heap = BinaryHeap::new();
    let mut ans = vec![];
    let mut index = 0;
    for i in 0..n {
        heap.push(Reverse((s[i], i)));
        if n - k <= i {
            while let Some(Reverse((c, i))) = heap.pop() {
                if index <= i {
                    index = i;
                    ans.push(c);
                    break;
                }
            }
        }
    }
    println!("{}", ans.iter().join(""));
}
