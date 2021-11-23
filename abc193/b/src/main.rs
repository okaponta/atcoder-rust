use std::{cmp::min, isize::MAX};

use proconio::input;

fn main() {
    input! {
       n:usize,
       apx:[(isize,isize,isize);n]
    }
    let mut ans = MAX;
    for e in apx {
        if e.2 - e.0 > 0 {
            ans = min(ans, e.1);
        }
    }
    println!("{}", if ans == MAX { -1 } else { ans });
}
