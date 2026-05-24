#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn case() {
    input! {
        s: Chars,
    }
    let mut count = HashMap::new();
    for c in s {
        *count.entry(c).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (c, i) in count {
        heap.push((i, c));
    }
    let mut ans = vec![];
    let mut banned = '#';
    while let Some((i, c)) = heap.pop() {
        if c == banned {
            if let Some((j, d)) = heap.pop() {
                ans.push(d);
                banned = d;
                if 1 < j {
                    heap.push((j - 1, d));
                }
                heap.push((i, c));
            } else {
                println!("No");
                return;
            }
        } else {
            ans.push(c);
            banned = c;
            if 1 < i {
                heap.push((i - 1, c));
            }
        }
    }
    println!("Yes");
    println!("{}", ans.iter().join(""));
}

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}
