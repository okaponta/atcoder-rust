use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut sum = 0;
    let mut ex_first = 0;
    for i in 0..n {
        sum += a[i];
        if i % 2 == 1 {
            ex_first += 2 * a[i];
        }
    }
    let mut ans = vec![0; n];
    ans[0] = sum - ex_first;
    for i in 1..n {
        ans[i] = 2 * a[i - 1] - ans[i - 1];
    }
    println!("{}", ans.iter().join(" "));
}
