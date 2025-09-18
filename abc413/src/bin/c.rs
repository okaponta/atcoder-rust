#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut qu = VecDeque::new();
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {c:usize, x:usize}
            qu.push_back((c, x));
        } else {
            input! {k:usize}
            let mut tmp = 0;
            let mut ans = 0;
            while let Some((c, x)) = qu.pop_front() {
                tmp += c;
                ans += c * x;
                if k <= tmp {
                    qu.push_front((tmp - k, x));
                    ans -= (tmp - k) * x;
                    break;
                }
            }
            println!("{ans}");
        }
    }
}
