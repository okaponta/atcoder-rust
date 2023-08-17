use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        td:[(usize,usize);n],
    }
    let mut map = HashMap::new();
    for (t, d) in td {
        (*map.entry(t).or_insert(vec![])).push(d);
    }
    let mut v = vec![];
    for (_, mut vv) in map {
        vv.sort();
        vv.reverse();
        v.push(vv);
    }
    v.sort();
    v.reverse();
    let mut ans = 0;
    let mut tmp = 0;
    let mut heap = BinaryHeap::new();
    for _ in 0..k {
        heap.push(Reverse(0));
    }
    for i in 0..v.len().min(k) {
        for j in 0..v[i].len() {
            if j == 0 {
                tmp += v[i][j];
                tmp += 2 * i + 1;
                heap.push(Reverse(!0));
            } else {
                heap.push(Reverse(v[i][j]));
                tmp += v[i][j];
            }
            while heap.len() != k {
                tmp -= heap.pop().unwrap().0;
            }
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
