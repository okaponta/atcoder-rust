use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    n = n.min(8);
    for i in 1..(1 << n) {
        let mut sum = 0;
        let mut b = vec![];
        for j in 0..n {
            if i >> j & 1 == 1 {
                sum += a[j];
                b.push(j + 1);
            }
        }
        if let Some(c) = map.insert(sum % 200, b.clone()) {
            println!("Yes");
            println!("{} {}", b.len(), b.iter().join(" "));
            println!("{} {}", c.len(), c.iter().join(" "));
            return;
        }
    }
    println!("No");
}
