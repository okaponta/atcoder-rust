use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[i64;n],
        p:[Usize1;n-1],
    }
    let mut map = HashMap::new();
    for i in 0..n - 1 {
        map.entry(p[i]).or_insert(vec![]).push(i + 1);
    }
    let mut h = vec![vec![0]];
    loop {
        let mut next = vec![];
        for i in &h[h.len() - 1] {
            if let Some(v) = map.get(&i) {
                for &nx in v {
                    next.push(nx);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        h.push(next);
    }
    for i in (0..h.len()).rev() {
        let mut diff = 0;
        for &j in &h[i] {
            diff += a[j];
        }
        if 0 < diff {
            println!("+");
            return;
        }
        if diff < 0 {
            println!("-");
            return;
        }
    }
    if a[0] == 0 {
        println!("0");
    } else if 0 < a[0] {
        println!("+");
    } else {
        println!("-");
    }
}
