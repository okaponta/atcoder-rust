use std::collections::{BinaryHeap, HashSet};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut a:[usize;n],
        mut b:[usize;n],
        mut c:[usize;n],
    }
    a.sort();
    b.sort();
    c.sort();
    a.reverse();
    b.reverse();
    c.reverse();
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    set.insert((0, 0, 0));
    heap.push((a[0] * b[0] + b[0] * c[0] + c[0] * a[0], 0, 0, 0));
    let mut count = 1;
    while let Some((s, ai, bi, ci)) = heap.pop() {
        if count == k {
            println!("{}", s);
            return;
        }
        count += 1;
        if !set.contains(&(ai + 1, bi, ci)) && ai + 1 < n {
            heap.push((
                a[ai + 1] * b[bi] + b[bi] * c[ci] + c[ci] * a[ai + 1],
                ai + 1,
                bi,
                ci,
            ));
            set.insert((ai + 1, bi, ci));
        }
        if !set.contains(&(ai, bi + 1, ci)) && bi + 1 < n {
            heap.push((
                a[ai] * b[bi + 1] + b[bi + 1] * c[ci] + c[ci] * a[ai],
                ai,
                bi + 1,
                ci,
            ));
            set.insert((ai, bi + 1, ci));
        }
        if !set.contains(&(ai, bi, ci + 1)) && ci + 1 < n {
            heap.push((
                a[ai] * b[bi] + b[bi] * c[ci + 1] + c[ci + 1] * a[ai],
                ai,
                bi,
                ci + 1,
            ));
            set.insert((ai, bi, ci + 1));
        }
    }
}
