use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut q = VecDeque::new();
    for a in a {
        q.push_front(a);
        while 1 < q.len() {
            let b = q.pop_front().unwrap();
            let c = q.pop_front().unwrap();
            if b == c {
                q.push_front(b + 1);
            } else {
                q.push_front(c);
                q.push_front(b);
                break;
            }
        }
    }
    println!("{}", q.len());
}
