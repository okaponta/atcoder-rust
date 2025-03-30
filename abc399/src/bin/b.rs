#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    let mut pi = vec![];
    for i in 0..n {
        pi.push((p[i], i));
    }
    pi.sort();
    pi.reverse();
    let mut map = HashMap::new();
    let mut ans = vec![0; n];
    for i in 0..n {
        if let Some(&j) = map.get(&pi[i].0) {
            ans[pi[i].1] = j;
        } else {
            ans[pi[i].1] = i + 1;
            map.insert(pi[i].0, i + 1);
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
