use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut set = HashSet::new();
    for i in 2..=n.sqrt() {
        let mut pow = i;
        while pow * i <= n {
            pow *= i;
            set.insert(pow);
        }
    }
    println!("{}", n - set.len());
}
