use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut sorted = a.clone();
    sorted.sort();
    for i in 0..k {
        let mut map = HashMap::new();
        let mut index = i;
        while index < n {
            *map.entry(a[index]).or_insert(0) += 1;
            *map.entry(sorted[index]).or_insert(0) -= 1;
            index += k;
        }
        if map.values().any(|v| v != &0) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
