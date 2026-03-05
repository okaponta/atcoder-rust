#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    let mut ca = 0;
    let mut cb = 0;
    let mut ans = 0;
    for c in s {
        if c == 'A' {
            ca += 1;
        } else if c == 'B' {
            if 0 < ca {
                ca -= 1;
                cb += 1;
            }
        } else {
            if 0 < cb {
                cb -= 1;
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
