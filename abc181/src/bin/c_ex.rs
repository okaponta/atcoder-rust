use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       xy: [(i32,i32);n]
    }
    if xy
        .iter()
        .tuple_combinations()
        .any(|(a, b, c)| (c.0 - a.0) * (b.1 - a.1) == (b.0 - a.0) * (c.1 - a.1))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
