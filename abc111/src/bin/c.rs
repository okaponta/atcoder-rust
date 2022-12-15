use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        v:[usize;n],
    }
    let mut odd = HashMap::new();
    let mut even = HashMap::new();
    for i in 0..n {
        if i % 2 == 1 {
            *odd.entry(v[i]).or_insert(0) += 1;
        } else {
            *even.entry(v[i]).or_insert(0) += 1;
        }
    }
    odd.insert(0, 0);
    even.insert(0, 0);
    let mut odds = odd.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    let mut evens = even.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    odds.sort();
    odds.reverse();
    evens.sort();
    evens.reverse();
    println!(
        "{}",
        n - (odds[0].0
            + if odds[0].1 == evens[0].1 {
                evens[1].0
            } else {
                evens[0].0
            })
        .max(
            odds[1].0
                + if odds[1].1 == evens[0].1 {
                    evens[1].0
                } else {
                    evens[0].0
                },
        )
    );
}
