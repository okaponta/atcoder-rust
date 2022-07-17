use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut dict = vec![vec![]; m + 1];
    for (i, &(a, b)) in ab.iter().enumerate() {
        dict[a].push(i);
        dict[b].push(i);
    }
    let mut r = 0;
    let mut map = HashMap::new();
    let mut ans = vec![0; m + 2];
    for l in 0..=m {
        loop {
            if r > m {
                break;
            }
            if map.len() == n {
                break;
            }
            for &x in &dict[r] {
                *map.entry(x).or_insert(0) += 1;
            }
            r += 1;
        }
        if map.len() == n {
            ans[r - l] += 1;
            ans[m + 1 - l] -= 1;
        }
        for &x in &dict[l] {
            *map.entry(x).or_insert(0) -= 1;
            if map.get(&x).unwrap() == &0 {
                map.remove(&x);
            }
        }
    }

    for i in 1..=m {
        ans[i] += ans[i - 1];
    }
    ans.remove(0);
    ans.remove(m);
    println!("{}", ans.iter().join(" "));
}
