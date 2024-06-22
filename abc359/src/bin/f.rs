use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut heap = BinaryHeap::new();
    a.sort();
    heap.push(Reverse((a[0], a[0], 0)));
    let mut ans = 0;
    for i in 1..n {
        if let Some(Reverse((cj, aj, j))) = heap.pop() {
            ans += cj;
            heap.push(Reverse(((2 * j + 3) * aj, aj, j + 1)));
            ans += a[i];
            heap.push(Reverse((3 * a[i], a[i], 1)));
        }
    }
    println!("{}", ans);
}
