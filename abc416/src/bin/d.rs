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
        m:usize,
        mut a:[usize;n],
        mut b:[usize;n],
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((a[i] % m, i));
    }
    for i in 0..n {
        b[i] %= m;
    }
    b.sort();
    let mut ans = 0;
    for b in b {
        if let Some(&(a, i)) = set.range(((m - b), 0)..).next() {
            set.remove(&(a, i));
            ans += (a + b) % m;
        } else {
            ans += b;
        }
    }
    set.into_iter().for_each(|i| ans += i.0);
    println!("{}", ans);
}
