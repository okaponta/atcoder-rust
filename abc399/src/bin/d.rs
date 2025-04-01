#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {
        n:usize,
        a:[usize;2*n],
    }
    let mut ng = HashSet::new();
    for i in 1..2 * n {
        if a[i] == a[i - 1] {
            ng.insert(a[i]);
        }
    }
    let mut pair = HashSet::new();
    let mut ans = 0;
    for i in 1..2 * n {
        if a[i] != a[i - 1] && !ng.contains(&a[i]) && !ng.contains(&a[i - 1]) {
            let tmp = (a[i].min(a[i - 1]), a[i].max(a[i - 1]));
            if pair.contains(&tmp) {
                ans += 1;
            }
        }
        if 2 <= i {
            pair.insert((a[i - 2].min(a[i - 1]), a[i - 2].max(a[i - 1])));
        }
    }
    println!("{}", ans);
}
