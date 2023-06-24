use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;7*n],
    }
    let mut ans = vec![0; n];
    for i in 0..7 * n {
        ans[i / 7] += a[i];
    }
    println!("{}", ans.iter().join(" "));
}
