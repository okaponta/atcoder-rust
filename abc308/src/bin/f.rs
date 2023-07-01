use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut p:[usize;n],
        l:[usize;m],
        d:[usize;m],
    }
    p.sort();
    let mut coupon = vec![];
    for i in 0..m {
        coupon.push((l[i], d[i]));
    }
    coupon.sort();
    let mut j = 0;
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..n {
        while j < m && coupon[j].0 <= p[i] {
            heap.push((coupon[j].1, j));
            j += 1;
        }
        ans += p[i];
        let c = heap.pop().unwrap_or((0, 0));
        ans -= c.0;
    }
    println!("{}", ans);
}
