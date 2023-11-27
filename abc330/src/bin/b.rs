use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
        a:[usize;n],
    }
    println!("{}", a.into_iter().map(|a| a.min(r).max(l)).join(" "));
}
