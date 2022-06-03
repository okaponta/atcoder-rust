use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        l:i64,
        a:[i64;n],
    }
    let mut heap = BinaryHeap::new();
    let sum = a.iter().sum::<i64>();
    for ai in a {
        heap.push(-ai);
    }
    if sum != l {
        heap.push(sum - l);
    }
    let mut ans = 0;
    while heap.len() > 1 {
        let join = heap.pop().unwrap() + heap.pop().unwrap();
        ans -= join;
        heap.push(join);
    }
    println!("{}", ans);
}
