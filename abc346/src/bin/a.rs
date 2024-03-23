use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = vec![];
    for i in 1..n {
        ans.push(a[i] * a[i - 1]);
    }
    println!("{}", ans.iter().join(" "));
}
