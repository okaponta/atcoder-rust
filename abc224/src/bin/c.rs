use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       xy:[(i64,i64);n]
    }
    println!(
        "{}",
        xy.iter()
            .tuple_combinations()
            .filter(|(a, b, c)| (c.0 - a.0) * (b.1 - a.1) != (b.0 - a.0) * (c.1 - a.1))
            .count()
    );
}
