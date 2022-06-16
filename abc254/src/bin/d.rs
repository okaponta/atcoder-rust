use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut set = HashSet::new();
    let mut sq = vec![];
    for i in 1..=n.sqrt() {
        sq.push(i * i);
    }
    for i in 1..sq.len() {
        for j in 0..i {
            let mut mul = 1;
            while mul * sq[i] <= n {
                set.insert((sq[i] * mul, sq[j] * mul));
                mul += 1;
            }
        }
    }
    println!("{}", set.len() * 2 + n);
}
