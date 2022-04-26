use std::collections::BTreeMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       h:usize,w:usize,n:usize,
       rca:[(Usize1,Usize1,usize);n],
    }
    let mut map = BTreeMap::new();
    rca.iter()
        .enumerate()
        .for_each(|(i, &(r, c, a))| map.entry(a).or_insert(vec![]).push((i, r, c)));
    let mut ans = vec![0; n];
    let mut rmax = vec![0; h];
    let mut cmax = vec![0; w];
    for v in map.values().rev() {
        for &(i, r, c) in v {
            ans[i] = rmax[r].max(cmax[c]);
        }
        for &(i, r, c) in v {
            rmax[r] = rmax[r].max(ans[i] + 1);
            cmax[c] = cmax[c].max(ans[i] + 1);
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
