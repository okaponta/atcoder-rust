use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut count = 0;
    for ai in a {
        count += 1;
        if !q.is_empty() && q.back().unwrap().0 == ai {
            // 同じ時
            let back = q.pop_back().unwrap();
            if back.1 + 1 == ai {
                //消えた時
                count -= ai;
            } else {
                //消えなかった時
                q.push_back((ai, back.1 + 1));
            }
        } else {
            // 違う時
            q.push_back((ai, 1));
        }
        println!("{}", count);
    }
}
