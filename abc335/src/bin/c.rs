use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut queue = VecDeque::new();
    for i in 0..n {
        queue.push_back((i as i64 + 1, 0i64));
    }
    for _ in 0..q {
        input! {q: u8}
        if q == 1 {
            input! {c: char}
            let mut x = queue[0].0;
            let mut y = queue[0].1;
            if c == 'R' {
                x += 1;
            }
            if c == 'L' {
                x -= 1;
            }
            if c == 'U' {
                y += 1;
            }
            if c == 'D' {
                y -= 1;
            }
            queue.pop_back();
            queue.push_front((x, y));
        } else {
            input! {p: Usize1}
            println!("{} {}", queue[p].0, queue[p].1);
        }
    }
}
