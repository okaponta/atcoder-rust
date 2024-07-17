use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        _t:usize,
        a:[usize;n],
    }
    let mut min = 0;
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    heap.push(a[n - 1]);
    for i in (0..n - 1).rev() {
        let tmp = heap.peek().unwrap().saturating_sub(a[i]);
        if min < tmp {
            min = tmp;
            ans = 1;
        } else if min == tmp {
            ans += 1;
        }
        heap.push(a[i]);
    }
    println!("{}", ans);
}
