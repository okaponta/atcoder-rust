use itertools::*;
use proconio::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut p = vec![vec![0]; n + 1];
    let mut p2 = vec![vec![0]; n + 2];
    for i in 0..n {
        p[a[i]].push(i + 1);
        p2[a[i]].push(i + 1);
        p2[a[i] - 1].push(i + 1);
    }
    let mut ans = 0;
    for i in 0..n {
        p[i].push(n + 1);
        p2[i].push(n + 1);
        ans += p[i].iter().tuple_windows().fold(0, f);
        ans -= p2[i].iter().tuple_windows().fold(0, f);
    }
    println!("{}", ans);
}

fn f(s: usize, lr: (&usize, &usize)) -> usize {
    s + (lr.1 - lr.0) * (lr.1 - lr.0 - 1) / 2
}
