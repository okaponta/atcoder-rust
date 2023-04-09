use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();
    set.insert(0);
    heap.push(Reverse(0));
    for a in a {
        let mut next = BinaryHeap::new();
        next.push(Reverse(0));
        let mut n_set = HashSet::new();
        n_set.insert(0);
        while next.len() <= k {
            let Reverse(min) = heap.pop().unwrap();
            if !n_set.contains(&min) {
                next.push(Reverse(min));
                n_set.insert(min);
            }
            if !set.contains(&(min + a)) {
                heap.push(Reverse(min + a));
                set.insert(min + a);
            }
        }
        heap = next;
    }
    let Reverse(ans) = heap.into_iter().last().unwrap();
    println!("{}", ans);
}
