#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
        t:[Chars;n],
    }
    let mut base = s.clone();
    let mut ans = 1 << 30;
    for i in 0..4 {
        let mut ss = base.clone();
        for i in 0..n {
            for j in 0..n {
                ss[j][n - i - 1] = base[i][j];
            }
        }
        let mut cnt = (i + 1) % 4;
        for i in 0..n {
            for j in 0..n {
                if t[i][j] != ss[i][j] {
                    cnt += 1;
                }
            }
        }
        base = ss;
        ans = ans.min(cnt);
    }
    println!("{}", ans);
}
