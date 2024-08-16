#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a:[i64;n],
        bk:[(i64,usize);q],
    }
    a.sort();
    for (b, k) in bk {
        let mut lower = -1;
        let mut upper = 1 << 60;
        while 1 < upper - lower {
            let mid = (lower + upper) / 2;
            let l = a.lower_bound(&(b - mid));
            let r = a.upper_bound(&(b + mid));
            if k <= r - l {
                upper = mid;
            } else {
                lower = mid;
            }
        }
        println!("{}", upper);
    }
}
