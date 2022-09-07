use std::collections::{BinaryHeap, HashMap};

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize);n],
    }
    let mut map = HashMap::new();
    for (a, b) in ab {
        (*map.entry(a).or_insert(vec![])).push(b);
    }
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for i in 1..=m {
        if let Some(v) = map.get(&i) {
            for b in v {
                heap.push(b);
            }
        }
        if let Some(b) = heap.pop() {
            ans += b;
        }
    }
    println!("{}", ans);
}
