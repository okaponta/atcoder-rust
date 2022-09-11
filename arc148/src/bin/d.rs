use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;2*n],
    }
    for i in 0..2 * n {
        a[i] %= m;
    }
    a.sort();
    let run_length = run_length_encode(a);
    let mut set = HashSet::new();
    let mut half = true;
    for (num, count) in run_length {
        if count % 2 == 0 {
            continue;
        }
        if m % 2 == 0 && set.remove(&((num + m / 2) % m)) {
            half = !half;
            continue;
        }
        set.insert(num);
    }
    println!(
        "{}",
        if set.len() == 0 && half {
            "Bob"
        } else {
            "Alice"
        }
    );
}

fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
