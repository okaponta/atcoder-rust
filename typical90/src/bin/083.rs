use std::collections::HashSet;

use num_integer::Roots;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
        q:usize,
        xy:[(Usize1,usize);q],
    }
    let mut g = vec![vec![]; n];
    let k = n.sqrt();
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut big = HashSet::new();
    for i in 0..n {
        if k < g[i].len() {
            big.insert(i);
        }
    }
    let mut h = vec![vec![]; n];
    for &(a, b) in &ab {
        if big.contains(&a) && big.contains(&b) {
            h[a].push(b);
            h[b].push(a);
        }
    }
    let mut color = vec![1; n];
    let mut last = vec![-1; n];
    for i in 0..q {
        if big.contains(&xy[i].0) {
            println!("{}", color[xy[i].0]);
            last[xy[i].0] = i as i64;
            color[xy[i].0] = xy[i].1;
            for &next in &h[xy[i].0] {
                color[next] = xy[i].1;
            }
        } else {
            let mut mxq = last[xy[i].0];
            for &next in &g[xy[i].0] {
                mxq = mxq.max(last[next]);
                if big.contains(&next) {
                    color[next] = xy[i].1;
                }
            }
            println!("{}", if mxq == -1 { 1 } else { xy[mxq as usize].1 });
            last[xy[i].0] = i as i64;
        }
    }
}
