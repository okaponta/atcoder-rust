use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[i64;n+1],
        mut c:[i64;n+m+1],
    }
    a.reverse();
    c.reverse();
    let mut b = vec![0; m + 1];
    for i in 0..=m {
        b[i] = c[i] / a[0];
        for j in 0..=n {
            c[i + j] -= b[i] * a[j];
        }
    }
    println!("{}", b.iter().rev().map(|i| i.to_string()).join(" "));
}
