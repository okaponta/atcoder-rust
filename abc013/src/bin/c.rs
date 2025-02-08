#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        h:usize,
        a:usize,
        b:usize,
        c:usize,
        d:usize,
        e:usize,
    }
    let mut ans = 1 << 60;
    for i in 0..=n {
        if h + b * i < e * (n - i) {
            continue;
        }
        if e * (n - i) < h + d * i {
            ans = ans.min(c * i);
            continue;
        }
        let mut lower = 0;
        let mut upper = i + 1;
        while upper - lower > 1 {
            let mid = (lower + upper) / 2;
            let x = h + b * mid + d * (i - mid);
            if e * (n - i) < x {
                upper = mid;
            } else {
                lower = mid;
            }
        }
        ans = ans.min(a * upper + c * (i - upper));
    }
    println!("{}", ans);
}
