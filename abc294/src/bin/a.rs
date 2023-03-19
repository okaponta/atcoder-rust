use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    println!("{}", a.into_iter().filter(|i| i % 2 == 0).join(" "));
}
