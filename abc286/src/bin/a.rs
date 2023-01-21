use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        pqrs:[Usize1;4],
        mut a:[usize;n],
    }
    for i in 0..=pqrs[1] - pqrs[0] {
        a.swap(pqrs[0] + i, pqrs[2] + i);
    }
    println!("{}", a.iter().join(" "));
}
