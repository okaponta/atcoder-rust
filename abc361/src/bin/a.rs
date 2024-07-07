use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        x:usize,
        mut a:[usize;n],
    }
    a.insert(k, x);
    println!("{}", a.iter().join(" "));
}
