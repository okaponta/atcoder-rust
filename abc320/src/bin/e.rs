use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        tws:[(usize,usize,usize);m],
    }
    let mut member = BinaryHeap::new();
    for i in 0..n {
        member.push(Reverse(i));
    }
    let mut heap = BinaryHeap::new();
    for (t, w, s) in tws {
        heap.push(Reverse((t, 1, w, s)));
    }
    let mut ans = vec![0; n];
    while let Some(Reverse((t, q, w, s))) = heap.pop() {
        if q == 0 {
            member.push(Reverse(w));
        } else {
            if let Some(Reverse(i)) = member.pop() {
                ans[i] += w;
                heap.push(Reverse((t + s, 0, i, 0)));
            }
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
