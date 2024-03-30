use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut ans = vec![];
    for a in a {
        if a % k == 0 {
            ans.push(a / k);
        }
    }
    println!("{}", ans.iter().join(" "));
}
