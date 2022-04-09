use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[u8;n],
    }
    let mut q = VecDeque::new();
    for ai in a {
        q.push_back(ai);
    }
    let mut flipped = false;
    while !q.is_empty() {
        let f = q.front().unwrap();
        let b = q.back().unwrap();
        if (!flipped && b == &0) || (flipped && b == &1) {
            q.pop_back();
        } else {
            if (!flipped && f == &0) || (flipped && f == &1) {
                q.pop_front();
                flipped = !flipped;
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
