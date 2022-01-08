use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       xy:[(i64,i64);n],
    }
    let square = xy
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)))
        .max()
        .unwrap() as f64;
    println!("{}", square.sqrt());
}
