use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        d:[usize;n],
    }
    println!(
        "{}",
        d.iter().tuple_combinations().fold(0, |s, (x, y)| s + x * y)
    );
}
