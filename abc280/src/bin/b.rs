use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut s:[i64;n],
    }
    let mut ans = vec![];
    s.insert(0, 0);
    for i in 1..=n {
        ans.push(s[i] - s[i - 1]);
    }
    println!("{}", ans.into_iter().join(" "));
}
