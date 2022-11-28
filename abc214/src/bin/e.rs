use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:usize,
    }
    (0..t).into_iter().for_each(|_| testcase());
}

fn testcase() {
    input! {
        n:usize,
        mut lr:[(usize,usize);n],
    }
    lr.push((1_000_000_001, 1_000_000_001));
    lr.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut heap = BinaryHeap::new();
    let mut cur = 0;
    for (l, r) in lr {
        let sabun = l - cur;
        heap.push(Reverse(r));
        for _ in 0..sabun {
            if heap.is_empty() {
                cur = l;
                break;
            }
            cur += 1;
            if let Some(Reverse(i)) = heap.pop() {
                //println!("{} {}", cur, i);
                if i < cur {
                    println!("No");
                    return;
                }
            }
        }
        //println!("{} {:?}", cur, heap);
    }
    println!("Yes");
}
