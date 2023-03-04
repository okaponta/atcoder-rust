use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut g = vec![HashSet::new(); n];
    let mut rev_g = vec![HashSet::new(); n];
    for (u, v) in uv {
        g[u].insert(v);
        rev_g[v].insert(u);
    }
    let mut count = 0;
    for i in 0..n {
        let mut tmp = vec![];
        for &j in &g[i] {
            for &k in &rev_g[i] {
                if j != k && !g[k].contains(&j) {
                    tmp.push((k, j));
                }
            }
        }
        for (k, j) in tmp {
            count += 1;
            if !g[k].contains(&j) {
                g[k].insert(j);
                rev_g[j].insert(k);
            }
        }
    }
    println!("{}", count);
}
