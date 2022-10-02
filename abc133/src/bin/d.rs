use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut sum = 0;
    let mut ex_last = 0;
    for i in 0..n {
        sum += a[i];
        if i % 2 == 0 && i != n - 1 {
            ex_last += 2 * a[i];
        }
    }
    let mut ans = vec![0; n];
    ans[n - 1] = sum - ex_last;
    for i in 0..n {
        ans[i] = 2 * a[(n + i - 1) % n] - ans[(n + i - 1) % n];
    }
    println!("{}", ans.iter().join(" "));
}
