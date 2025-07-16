#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    let mut ud = vec![(0, 'D')];
    for i in 1..n - 1 {
        if p[i - 1] < p[i] && p[i] > p[i + 1] {
            ud.push((i, 'U'));
        } else if p[i - 1] > p[i] && p[i] < p[i + 1] {
            ud.push((i, 'D'));
        }
    }
    ud.push((n, 'D'));
    let mut ans = 0;
    for i in 1..ud.len() - 2 {
        if ud[i].1 == 'U' && ud[i + 1].1 == 'D' {
            let a = if ud[i - 1].1 == 'D' {
                ud[i].0 - ud[i - 1].0
            } else {
                ud[i].0 - ud[i - 1].0 - 1
            };
            let b = if ud[i + 2].1 == 'U' {
                ud[i + 2].0 - ud[i + 1].0
            } else {
                ud[i + 2].0 - ud[i + 1].0 - 1
            };
            ans += a * b;
        }
    }
    println!("{}", ans);
}
