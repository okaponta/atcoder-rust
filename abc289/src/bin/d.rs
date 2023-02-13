use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        m:usize,
        b:[usize;m],
        x:usize,
    }
    let ng = b.into_iter().collect::<HashSet<_>>();
    let mut ok = vec![false; x + 1];
    ok[0] = true;
    for i in 0..x {
        if !ok[i] {
            continue;
        }
        for j in 0..n {
            let next = i + a[j];
            if ng.contains(&next) || x < next {
                continue;
            }
            ok[next] = true;
        }
    }
    println!("{}", if ok[x] { "Yes" } else { "No" });
}
