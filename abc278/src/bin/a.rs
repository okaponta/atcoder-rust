use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut a:[usize;n],
    }
    for _ in 0..k {
        a.remove(0);
        a.push(0);
    }
    println!("{}", a.iter().join(" "));
}
