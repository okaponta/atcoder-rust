use std::collections::HashSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut p:[Usize1;n-1],
    }
    p.insert(0, n + 1);
    let mut childs = vec![0; n];
    for i in 1..n {
        childs[p[i]] += 1;
    }
    for _ in 0..q {
        input! {
            m:usize,
            v:[Usize1;m],
        }
        let set = v.iter().collect::<HashSet<_>>();
        let mut ans = 0;
        for i in 0..m {
            ans += childs[v[i]] + 1;
            if set.contains(&p[v[i]]) {
                ans -= 2;
            }
        }
        println!("{}", ans);
    }
}
