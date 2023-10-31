use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n:usize,
        td:[(usize,usize);n],
    }
    let mut map = BTreeMap::new();
    for (t, d) in td {
        (*map.entry(t).or_insert(vec![])).push(d);
    }
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    let mut t = 0;
    loop {
        if heap.is_empty() {
            if let Some(t_next) = map.range(t..).next() {
                t = *t_next.0;
            } else {
                break;
            }
        }
        if map.contains_key(&t) {
            for v in map[&t].iter() {
                heap.push(Reverse(t + v));
            }
        }
        while let Some(Reverse(ti)) = heap.pop() {
            if t <= ti {
                ans += 1;
                break;
            }
        }
        t += 1;
    }
    println!("{}", ans);
}
