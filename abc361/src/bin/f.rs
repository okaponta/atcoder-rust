use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:u128,
    }
    let mut ans = n.sqrt();
    let mut used = HashSet::new();
    let mut i = 2;
    while i * i * i <= n {
        let mut tmp = i * i * i;
        while tmp <= n {
            if !used.contains(&tmp) && tmp.sqrt() * tmp.sqrt() != tmp {
                ans += 1;
                used.insert(tmp);
            }
            tmp *= i * i;
        }
        i += 1;
    }
    println!("{}", ans);
}
