use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        _n:usize,
        d:usize,
        k:usize,
        lr:[(Usize1,Usize1);d],
        st:[(Usize1,Usize1);k],
    }
    for (s, t) in st {
        if s < t {
            let mut left = s;
            for i in 0..d {
                if lr[i].0 <= left && left < lr[i].1 {
                    left = lr[i].1;
                    if t <= left {
                        println!("{}", i + 1);
                        break;
                    }
                }
            }
        } else {
            let mut right = s;
            for i in 0..d {
                if lr[i].0 < right && right <= lr[i].1 {
                    right = lr[i].0;
                    if right <= t {
                        println!("{}", i + 1);
                        break;
                    }
                }
            }
        }
    }
}
