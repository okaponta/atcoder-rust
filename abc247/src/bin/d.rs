use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        query:usize,
    }
    let mut q = VecDeque::new();
    for _ in 0..query {
        input! {i: usize}
        if i == 1 {
            input! {x: i64, c:i64}
            q.push_back((x, c));
        } else {
            input! {mut c: i64}
            let mut ans = 0;
            while c > 0 {
                let (xx, mut cc) = q.pop_front().unwrap();
                if c > cc {
                    ans += xx * cc;
                    c -= cc;
                    continue;
                }
                ans += xx * c;
                cc -= c;
                c -= c;
                q.push_front((xx, cc));
            }
            println!("{}", ans);
        }
    }
}
