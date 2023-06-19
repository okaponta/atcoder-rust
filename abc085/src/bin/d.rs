use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        h:usize,
        ab:[(usize,usize);n],
    }
    let mut heap = BinaryHeap::new();
    for (a, b) in ab {
        heap.push((a, false));
        heap.push((b, true));
    }
    let mut count = 0;
    let mut d = 0;
    while let Some((damage, throw)) = heap.pop() {
        if throw {
            d += damage;
            count += 1;
        } else {
            let c = (h - d + damage - 1) / damage;
            count += c;
            d += c * damage;
        }
        if h <= d {
            break;
        }
    }
    println!("{}", count);
}
